# 0.2

> Fri June 7th 2019

  - Add support for `feature(try_blocks)`. Per-se, this feature doesn’t require any special support
    from `try-guard` — i.e. there’s no difference between the *stable* and *nightly* codes. However,
    the change is done at usage and a *slightly*, internal change was required to make it possible.
    Because of such a change, the `tray-trait` feature gate is not needed anymore and you then get
    `guard!` usable with `Option`, `Result` or whatever implements the right interface.
  - Segregate the nightly code from the stable code in tests.
  - Add the `verify!` macro. That macro behaves a bit like `guard!` but returns `Some(())` or `None`
    based on its predicate truth. This is useful when you want to manipulate the error in a more
    direct way and use the `?` operator.
  - Add the `test-nightly` feature gate. This allows to test some parts of the crate that do require
    a nightly **rustc**.

## 0.1.1

  - Fix a typo in the front page documentation.

# 0.1

  - Initial revision.
