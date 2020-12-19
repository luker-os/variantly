extern crate variantly_derive;

pub use variantly_derive::Variantly;

#[macro_export]
macro_rules! and {
    ($variant:path, $enum_a:expr, $enum_b:expr) => {
        match (&$enum_a, $enum_b) {
            (&$variant(_), $variant(val)) => $variant(val),
            _ => $enum_a,
        }
    };
}

#[macro_export]
macro_rules! and_then {
    ($variant:path, $enum:expr, $and_then:tt) => {
        match $enum {
            $variant(value) => $variant($and_then(value)),
            _ => $enum,
        }
    };
}

#[macro_export]
macro_rules! contains {
    ($variant:path, $enum:expr, $target:expr) => {
        match $enum {
            $variant(value) => value == $target,
            _ => false,
        }
    };
}

#[macro_export]
macro_rules! expect {
    ($variant:path, $enum:expr, $msg:expr) => {
        variantly::unwrap_or_else!($variant, $enum, (|| panic!("{}", $msg)))
    };
}

#[macro_export]
macro_rules! is {
    ($variant:path, $enum:expr) => {
        match $enum {
            $variant(_) => true,
            _ => false,
        }
    };
}

#[macro_export]
macro_rules! replace {
    ($variant:path, $enum:expr, $value:expr) => {
        std::mem::replace($enum, $variant($value))
    };
}

#[macro_export]
macro_rules! ok {
    ($variant:path, $enum:expr) => {
        match $enum {
            $variant(value) => Some(value),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! ok_or {
    ($variant:path, $enum:expr, $err:expr) => {
        variantly::ok_or_else!($variant, $enum, (|| $err))
    };
}

#[macro_export]
macro_rules! ok_or_else {
    ($variant:path, $enum:expr, $else:tt) => {
        match $enum {
            $variant(value) => Ok(value),
            _ => Err($else()),
        }
    };
}

#[macro_export]
macro_rules! or {
    ($variant:path, $enum_a:expr, $enum_b:expr) => {
        match $enum_a {
            $variant(val) => $variant(val),
            _ => $enum_b,
        }
    };
}

#[macro_export]
macro_rules! or_else {
    ($variant:path, $enum:expr, $or_else:tt) => {
        match $enum {
            $variant(val) => $variant(val),
            _ => $variant($or_else()),
        }
    };
}

#[macro_export]
macro_rules! unwrap {
    ($variant:path, $enum:expr) => {
        variantly::unwrap_or_else!($variant, $enum, (|| panic!()))
    };
}

#[macro_export]
macro_rules! unwrap_or {
    ($variant:path, $enum:expr, $or:expr) => {
        variantly::unwrap_or_else!($variant, $enum, (|| $or))
    };
}

#[macro_export]
macro_rules! unwrap_or_else {
    ($variant:path, $enum:expr, $else:tt) => {
        match $enum {
            $variant(value) => value,
            _ => $else(),
        }
    };
}
