//! The [`guard!`] macro.
//!
//! The [`guard!`] macro implements a control-flow sugar that occurs very often in common Rust code:
//!
//! ```rust
//! fn foo(cond: bool) -> Option<i32> {
//!   if !cond {
//!     return None;
//!   }
//!
//!   // do something useful
//!
//!   Some(42)
//! }
//! ```
//!
//! This pattern of testing arguments and early-returning with an error is very typical.
//! Unfortunately, the [`?`] operator doesn’t help us here because we want to early-return on a
//! boolean value, not an error value.
//!
//! A not very idiomatic and weird way to rewrite that:
//!
//! ```rust
//! fn foo(cond: bool) -> Option<i32> {
//!   if cond { Some(()) } else { None }?;
//!   Some(42)
//! }
//! ```
//!
//! This crate provides the [`guard!`] macro — analoguous to the [`guard`] Haskell `Alternative`
//! function — that helps early-return from a function if a predicate is `false`:
//!
//! ```rust
//! # #![cfg_attr(feature = "test-nightly", feature(try_trait))]
//! # #[cfg(feature = "test-nightly")] mod lol {
//! use try_guard::guard;
//!
//! fn foo(cond: bool) -> Option<i32> {
//!   guard!(cond);
//!   Some(42)
//! }
//! # }
//! ```
//!
//! ## Custom guard types
//!
//! This crate also allows you to _guard_ to anything that implements [`Try<Error = NoneError>`] or
//! `From<NoneError>` (nightly only).
//!
//! For instance, the following works:
//!
//! ```rust
//! # #![cfg_attr(feature = "test-nightly", feature(try_trait))]
//! # #[cfg(feature = "test-nightly")] mod lol {
//! use std::ops::Try;
//! use std::option::NoneError;
//! use try_guard::guard;
//!
//! #[derive(Clone, Debug, Eq, PartialEq)]
//! enum MyGuard<T> {
//!   Just(T),
//!   Nothing
//! }
//!
//! impl<T> MyGuard<T> {
//!   fn new(x: T) -> Self {
//!     MyGuard::Just(x)
//!   }
//!
//!   fn none() -> Self {
//!     MyGuard::Nothing
//!   }
//! }
//!
//! impl<T> Try for MyGuard<T> {
//!   type Ok = T;
//!
//!   type Error = NoneError;
//!
//!   fn from_error(_: Self::Error) -> Self {
//!     MyGuard::none()
//!   }
//!
//!   fn from_ok(x: Self::Ok) -> Self {
//!     MyGuard::new(x)
//!   }
//!
//!   fn into_result(self) -> Result<Self::Ok, Self::Error> {
//!     match self {
//!       MyGuard::Just(x) => Ok(x),
//!       MyGuard::Nothing => Err(NoneError)
//!     }
//!   }
//! }
//!
//! fn foo(cond: bool) -> MyGuard<i32> {
//!   guard!(cond);
//!   MyGuard::new(42)
//! }
//!
//! fn main() {
//!   assert_eq!(foo(false), MyGuard::Nothing);
//! }
//! # }
//! ```
//!
//! ## More control on the error type
//!
//! If you’d rather manipulate the error type when the predicate is false, you might be interested
//! in the [`verify!`] macro instead. That macro is akin to [`guard!`] but instead doesn’t exit the
//! current scope: it maps the predicate’s truth to either `Some(())` or `None`, allowing you to
//! call `Option::ok_or` or whatever error combinator you want to.
//!
//! ```rust
//! use try_guard::verify;
//!
//! fn foo(cond: bool) -> Result<u32, String> {
//!   verify!(cond).ok_or("bad condition".to_owned())?;
//!   Ok(123)
//! }
//! ```
//!
//! ## Feature flags
//!
//!   - The `test-nightly` feature flag can be used to test nightly-related features that come
//!     freely and don’t require a nightly build of rustc to compile this crate but require one at
//!     use site.
//!
//! [`guard!`]: guard
//! [`verify!`]: verify
//! [`guard`]: http://hackage.haskell.org/package/base-4.12.0.0/docs/Control-Monad.html#v:guard
//! [`?`]: https://doc.rust-lang.org/std/ops/trait.Try.html
//! [`Try<Error = NoneError>`]: https://doc.rust-lang.org/std/ops/trait.Try.html

/// The [`guard!`] macro.
///
/// [`guard!`]: guard
#[macro_export]
macro_rules! guard {
  ($e:expr) => {
    if !$e {
      None?
    }
  };
}

/// A version of [`guard!`] that doesn’t shortcut.
///
/// The advantage of this macro over [`guard!`] is to allow you to manipulate the resulting
/// [`Option`].
///
/// [`guard!`]: guard
#[macro_export]
macro_rules! verify {
  ($e:expr) => {
    if !$e {
      None
    } else {
      Some(())
    }
  }
}
