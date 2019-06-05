#![cfg_attr(feature = "test-nightly", feature(try_trait), feature(try_blocks))]

use try_guard::verify;

#[test]
fn verify_success() {
  let foo = verify!(1 < 2);
  assert_eq!(foo, Some(()));
}

#[test]
fn verify_failure() {
  let foo = verify!(1 > 2);
  assert_eq!(foo, None);
}

#[cfg(feature = "test-nightly")]
mod nightly {
  use super::*;

  #[test]
  fn verify_try_success() {
    let foo: Option<()> = try {
      verify!(1 < 2)?
    };
    assert_eq!(foo, Some(()));
  }

  #[test]
  fn verify_try_failure() {
    let foo: Option<()> = try {
      verify!(1 > 2)?
    };
    assert_eq!(foo, None);
  }
}
