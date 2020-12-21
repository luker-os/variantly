extern crate variantly_derive;

pub use variantly_derive::Variantly;

/// Compare two enum variants and if they are of the passed in enum variant, return the second. Otherwise, return the first.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum_a` - An instance of an enum of the same base type as `variant`
/// * `enum_b` - An instance of an enum of the same base type as `variant`
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp_a = Temperature::F(32);
/// let temp_b = Temperature::F(212);
/// assert_eq!(
///     variantly::and!(Temperature::F, temp_a, temp_b),
///     temp_b
/// );
///
/// let temp_c = Temperature::C(100);
/// assert_eq!(
///     variantly::and!(Temperature::F, temp_a, temp_c),
///     temp_a
/// );
/// ```
#[macro_export]
macro_rules! and {
    ($variant:path, $enum_a:expr, $enum_b:expr) => {
        match (&$enum_a, $enum_b) {
            (&$variant(_), $variant(val)) => $variant(val),
            _ => $enum_a,
        }
    };
}

/// If the given enum is of the passed in enum variant, perform the passed in operation on the contained value.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
/// * `and_then` - A function that accepts and returns a value of the same type as the inner value of the passed in enum variant
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp_a = Temperature::F(32);
/// let f_boil = |value| value + 180;
/// let temp_a = variantly::and_then!(Temperature::F, temp_a, f_boil);
/// assert_eq!(temp_a, Temperature::F(212));
///
/// let temp_b = Temperature::C(0);
/// let temp_b = variantly::and_then!(Temperature::F, temp_b, f_boil);
/// assert_eq!(temp_b, Temperature::C(0));
/// ```
#[macro_export]
macro_rules! and_then {
    ($variant:path, $enum:expr, $and_then:tt) => {
        match $enum {
            $variant(value) => $variant($and_then(value)),
            _ => $enum,
        }
    };
}

/// Return the inner value of an enum instance if it is of the given variant, otherwise panic with the given message.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
/// * `msg` - The message to include if panic!() is called due to an unexpected enum variant.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp = Temperature::F(32);
/// assert_eq!(32, variantly::expect!(Temperature::F, temp, "This should have been in fahrenheit"))
/// ```
/// ```should_panic
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp = Temperature::C(0);
/// variantly::expect!(Temperature::F, temp, "This should have been in fahrenheit");
/// ```
#[macro_export]
macro_rules! expect {
    ($variant:path, $enum:expr, $msg:expr) => {
        variantly::unwrap_or_else!($variant, $enum, (|| panic!("{}", $msg)))
    };
}

/// Determines if an enum is an instance of a given variant.
///
/// # Arguments
///
/// * `variant_pattern` - A pattern to test against to determine if the enum is of the expected variant.
/// _Note: This interface is slightly different than other declarative macros in this crate.
/// This allows for testing against enum variants with different syntactical structures._
/// * `enum` - An instance of an enum of the same base type as `variant`
///
/// # Examples
/// ```
/// enum Color {
///     Red,
///     Green,
///     Other(u32, u32, u32)
/// }
///
/// let color = Color::Red;
/// assert!(variantly::is!(Color::Red, color));
///
/// let color = Color::Other(40, 128, 180);
/// assert!(variantly::is!(Color::Other(_, _, _), color));
/// ```
#[macro_export]
macro_rules! is {
    ($variant_pattern:pat, $enum:expr) => {
        match $enum {
            $variant_pattern => true,
            _ => false,
        }
    };
}

/// Convert the passed in enum into an Option.
/// If the enum is of the expected variant it will result in Some(value), otherwise it will result in None.
///
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp = Temperature::F(32);
/// assert_eq!(Some(32), variantly::ok!(Temperature::F, temp));
/// assert_eq!(None, variantly::ok!(Temperature::C, temp));
/// ```
#[macro_export]
macro_rules! ok {
    ($variant:path, $enum:expr) => {
        match $enum {
            $variant(value) => Some(value),
            _ => None,
        }
    };
}

/// Convert the passed in enum into an Result.
/// If the enum is of the expected variant it will result in Ok(value), otherwise it will result in the passed in error.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
/// * `err` - An instance of an Error to return if the passed in `enum` is not the expected `variant`
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp = Temperature::K(373);
/// let err = "This isn't Kelvin!";
/// assert_eq!(Ok(373), variantly::ok_or!(Temperature::K, temp, err));
///
/// let temp = Temperature::C(100);
/// assert_eq!(Err(err), variantly::ok_or!(Temperature::K, temp, err));
/// ```
#[macro_export]
macro_rules! ok_or {
    ($variant:path, $enum:expr, $err:expr) => {
        variantly::ok_or_else!($variant, $enum, (|| $err))
    };
}

/// Convert the passed in enum into an Result.
/// If the enum is of the expected variant it will result in Ok(value), otherwise it will result in an error constructed by the passed in expression.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
/// * `else` - An expression that returns an instance of an Error to return if the passed in `enum` is not the expected `variant`.
/// _`else` is lazily evaluated._
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp = Temperature::K(373);
/// let or_else = || "This isn't Kelvin!";
/// assert_eq!(Ok(373), variantly::ok_or_else!(Temperature::K, temp, or_else));
///
/// let temp = Temperature::C(100);
/// assert_eq!(Err(or_else()), variantly::ok_or_else!(Temperature::K, temp, or_else));
/// ```
#[macro_export]
macro_rules! ok_or_else {
    ($variant:path, $enum:expr, $else:tt) => {
        match $enum {
            $variant(value) => Ok(value),
            _ => Err($else()),
        }
    };
}

/// Compare two enum variants and if the first is of the expected variant, return it. Otherwise return the second variant.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum_a` - An instance of an enum of the same base type as `variant`
/// * `enum_b` - An instance of an enum of the same base type as `variant`
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp_a = Temperature::F(32);
/// let temp_b = Temperature::F(212);
/// assert_eq!(
///     variantly::or!(Temperature::F, temp_a, temp_b),
///     temp_a
/// );
///
/// let temp_c = Temperature::C(100);
/// assert_eq!(
///     variantly::or!(Temperature::C, temp_a, temp_c),
///     temp_c
/// );
/// ```
#[macro_export]
macro_rules! or {
    ($variant:path, $enum_a:expr, $enum_b:expr) => {
        match $enum_a {
            $variant(val) => $variant(val),
            _ => $enum_b,
        }
    };
}

/// If the given enum is of the expected variant, return it. Otherwise calculate a fallback value from the passed in fn or closure.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
/// * `or_else` - A fn or closure that computes a default value.
/// _`or_else` is lazily evaluated._
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp = Temperature::F(32);
/// let fallback = || 0;
/// assert_eq!(
///     variantly::or_else!(Temperature::F, temp, fallback),
///     temp
/// );
///
/// let temp = Temperature::C(100);
/// assert_eq!(
///     variantly::or_else!(Temperature::F, temp, fallback),
///     Temperature::F(0)
/// );
/// ```
#[macro_export]
macro_rules! or_else {
    ($variant:path, $enum:expr, $or_else:tt) => {
        match $enum {
            $variant(val) => $variant(val),
            _ => $variant($or_else()),
        }
    };
}

/// If the given enum is of the expected variant, replace it's inner value and return the previously contained value. Otherwise return `value`.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
/// * `value` - The value to insert into the enum instance if it is of the expected variant.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let mut temp = Temperature::F(32);
/// let original = variantly::replace!(Temperature::F, &mut temp, 100);
/// assert_eq!(Temperature::F(100), temp);
/// assert_eq!(Temperature::F(32), original);
///
/// let mut temp = Temperature::C(0);
/// let new_value = variantly::replace!(Temperature::F, &mut temp, 100);
/// assert_eq!(Temperature::C(0), temp);
/// assert_eq!(Temperature::F(100), new_value);
/// ```
#[macro_export]
macro_rules! replace {
    ($variant:path, $enum:expr, $value:expr) => {{
        let new_variant_value = $variant($value);
        match $enum {
            $variant(_) => std::mem::replace($enum, new_variant_value),
            _ => new_variant_value,
        }
    }};
}

/// Return the value contained by the enum instance if it is of the expected variant, otherwise panics.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp = Temperature::F(32);
/// assert_eq!(32, variantly::unwrap!(Temperature::F, temp))
///
/// ```
/// ```should_panic
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp = Temperature::F(32);
/// variantly::unwrap!(Temperature::C, temp);
///
/// ```
#[macro_export]
macro_rules! unwrap {
    ($variant:path, $enum:expr) => {
        variantly::unwrap_or_else!($variant, $enum, (|| panic!()))
    };
}

/// Return the value contained by the enum instance if it is of the expected variant, otherwise returns a passed in default value.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
/// * `or` - The value to return if `enum` is not of the expected variant
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
///
/// let temp = Temperature::F(32);
/// assert_eq!(32, variantly::unwrap_or!(Temperature::F, temp, 100));
/// assert_eq!(100, variantly::unwrap_or!(Temperature::C, temp, 100));
///
/// ```
#[macro_export]
macro_rules! unwrap_or {
    ($variant:path, $enum:expr, $or:expr) => {
        variantly::unwrap_or_else!($variant, $enum, (|| $or))
    };
}

/// Return the value contained by the enum instance if it is of the expected variant, otherwise computes a default value using a passed in fn or closure.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
/// * `or_else` - A fn or closure that computes a default value
/// _`or_else` is lazily evaluated_
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Temperature {
///     C(u32),
///     F(u32),
///     K(u32),
/// }
/// fn expensive_calculation() -> u32 {
///     100
/// }
///
/// let temp = Temperature::F(32);
/// assert_eq!(32, variantly::unwrap_or_else!(Temperature::F, temp, expensive_calculation));
/// assert_eq!(100, variantly::unwrap_or_else!(Temperature::C, temp, expensive_calculation));
///
/// ```
#[macro_export]
macro_rules! unwrap_or_else {
    ($variant:path, $enum:expr, $else:tt) => {
        match $enum {
            $variant(value) => value,
            _ => $else(),
        }
    };
}
