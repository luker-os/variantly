#[macro_use]
extern crate variantly_derive;

pub use variantly_derive::Variantly;

// TODO ordering of `$variant:path` should be consistent in macro signatures

#[macro_export]
macro_rules! replace {
    ($enum:expr, $variant:path, $value:expr) => {
        std::mem::replace($enum, $variant($value))
    };
}

#[macro_export]
macro_rules! or {
    ($a:expr, $b:expr, $variant:path) => {
        match (&$a, $b) {
            (&$variant(_), $variant(val)) => $variant(val),
            _ => $a,
        }
    };
}

#[macro_export]
macro_rules! or_else {
    ($a:expr, $or_else:tt, $variant:path) => {
        match $a {
            $variant(value) => $variant($or_else(value)),
            _ => $a,
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_else {
    ($enum:expr, $variant:path, $else:tt) => {
        match $enum {
            $variant(value) => value,
            _ => $else(),
        }
    };
}

#[macro_export]
macro_rules! unwrap_or {
    ($enum:expr, $variant:path, $or:expr) => {
        variantly::unwrap_or_else!($enum, $variant, (|| $or))
    };
}

#[macro_export]
macro_rules! unwrap {
    ($enum:expr, $variant:path) => {
        variantly::unwrap_or_else!($enum, $variant, (|| panic!()))
    };
}

#[macro_export]
macro_rules! contains {
    ($enum:expr, $variant:path, $target:expr) => {
        match $enum {
            $variant(value) => value == $target,
            _ => false,
        }
    };
}

#[macro_export]
macro_rules! expect {
    ($enum:expr, $variant:path, $msg:expr) => {
        variantly::unwrap_or_else!($enum, $variant, (|| panic!("{}", $msg)))
    };
}

#[macro_export]
macro_rules! is {
    ($enum:expr, $variant:path) => {
        match $enum {
            $variant(_) => true,
            _ => false,
        }
    };
}

#[macro_export]
macro_rules! ok {
    ($enum:expr, $variant:path) => {
        match $enum {
            $variant(value) => Some(value),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! ok_or {
    ($enum:expr, $variant:path, $err:expr) => {
        variantly::ok_or_else!($enum, $variant, (|| $err))
    };
}

#[macro_export]
macro_rules! ok_or_else {
    ($enum:expr, $variant:path, $else:tt) => {
        match $enum {
            $variant(value) => Ok(value),
            _ => Err($else()),
        }
    };
}
