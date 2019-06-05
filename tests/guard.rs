#![cfg_attr(feature = "test-nightly", feature(try_trait), feature(try_blocks))]

use try_guard::guard;

#[test]
fn success() {
  fn foo() -> Option<i32> {
    guard!(1 < 2);
    Some(10)
  }

  assert_eq!(foo(), Some(10));
}

#[test]
fn failure() {
  fn foo() -> Option<i32> {
    guard!(1 > 2);
    Some(10)
  }

  assert_eq!(foo(), None);
}

#[cfg(feature = "test-nightly")]
mod nightly {
  use super::*;

  #[test]
  fn try_success() {
    let foo: Option<i32> = try {
      guard!(1 < 2);
      10
    };

    assert_eq!(foo, Some(10));
  }

  #[test]
  fn try_failure() {
    let foo: Option<i32> = try {
      guard!(1 > 2);
      10
    };

    assert_eq!(foo, None);
  }

  #[derive(Debug, PartialEq)]
  struct CustomError;

  impl From<std::option::NoneError> for CustomError {
    fn from(_: std::option::NoneError) -> Self {
      CustomError
    }
  }

  #[test]
  fn try_result_success() {
    let foo: Result<i32, CustomError> = try {
      guard!(1 < 2);
      10
    };

    assert_eq!(foo, Ok(10));
  }

  #[test]
  fn try_result_failure() {
    let foo: Result<i32, CustomError> = try {
      guard!(1 > 2);
      10
    };

    assert_eq!(foo, Err(CustomError));
  }
}
