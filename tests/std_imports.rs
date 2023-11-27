#![allow(unused)]
use variantly::Variantly;

/// Test compilation while custom Result/Option/etc in scope.
enum Result {
    Ok,
    Err,
}
use self::Result::*;

enum Option {
    Some,
    None,
}
use self::Option::*;

trait FnOnce {}

macro_rules! panic {
    () => {};
}

#[derive(Variantly)]
enum TestPreludeImportConflict {
    Variant(u32),
}

#[no_implicit_prelude]
mod no_std {
    use ::std;
    use ::variantly::Variantly;

    // Test compilation w/o access to prelude contents.
    #[derive(Variantly)]
    enum TestIsolatedDerive {
        Variant(u32),
    }
}
