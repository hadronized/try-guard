# The `guard` Haskell `Alternative` function brought to Rust

<!-- cargo-sync-readme start -->

The [`guard!`] macro.

The [`guard!`] macro implements a control-flow sugar that occurs very often in common Rust code:

```
fn foo(cond: bool) -> Option<i32> {
  if !cond {
    return None;
  }

  // do something useful

  Some(42)
}
```

This pattern of testing arguments and early-returning with an error is very typical.
Unfortunately, the [`?`] operator doesn’t help us here because we want to early-return on a
boolean value, not an error value.

A not very idiomatic and weird way to rewrite that:

```
fn foo(cond: bool) -> Option<i32> {
  if cond { Some(()) } else { None }?;
  Some(42)
}
```

This crate provides the [`guard!`] macro — analoguous to the [`guard`] Haskell `Alternative`
function — that helps early-return from a function if a predicate is `false`:

```
#![feature(try_trait)]

use try_guard::guard;

fn foo(cond: bool) -> Option<i32> {
  guard!(cond);
  Some(42)
}
```

## Custom guard types

This crate also allows you to _guard_ to any time that implements [`Try<Error = NoneError>`]. For
instance, the following works:

```
#![feature(try_trait)]

use std::ops::Try;
use std::option::NoneError;
use try_guard::guard;

enum MyGuard<T> {
  Just(T),
  Nothing
}

impl<T> MyGuard<T> {
  fn new(x: T) -> Self {
    MyGuard::Just(x)
  }

  fn none() -> Self {
    MyGuard::Nothing
  }
}

impl<T> Try for MyGuard<T> {
  type Ok = T;

  type Error = NoneError;

  fn from_error(_: Self::Error) -> Self {
    MyGuard::none()
  }

  fn from_ok(x: Self::Ok) -> Self {
    MyGuard::new(x)
  }

  fn into_result(self) -> Result<Self::Ok, Self::Error> {
    match self {
      MyGuard::Just(x) => Ok(x),
      MyGuard::Nothing => Err(NoneError)
    }
  }
}

fn foo(cond: bool) -> MyGuard<i32> {
  guard!(cond);
  MyGuard::new(42)
}

```

# Feature flags

  - The `"try-trait"` flag allows to use `guard!` with any type that implements
    [`Try<Error = NoneError>`]. Disabling this will make `guard!` work only with
    [`Option`]. **Enabled by default.**
    - **This feature currently requires a nightly build.**

[`guard!`]: guard
[`guard`]: http://hackage.haskell.org/package/base-4.12.0.0/docs/Control-Monad.html#v:guard
[`?`]: https://doc.rust-lang.org/std/ops/trait.Try.html
[`Try<Error = NoneError>`]: https://doc.rust-lang.org/std/ops/trait.Try.html

<!-- cargo-sync-readme end -->
