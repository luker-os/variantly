extern crate variantly_derive;

pub use variantly_derive::Variantly;

/// Compare two enum variants and if they are of the passed in enum variant, return the second. Otherwise, return the first.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum_a` - An instance of an enum of the same base type as `variant`
/// * `enum_b` - An instance of an enum of the same base type as `variant`
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Location {
///     Coord(u32, u32),
///     ID(u32),
///     Unknown
/// }
/// //  Multi-Variable
/// let location_a = Location::Coord(123, 456);
/// let location_b = Location::Coord(456, 789);
/// //  If both match the intended variant, the second is returned.
/// assert_eq!(
///     variantly::and!(Location::Coord, location_a, location_b, (a, b)),
///     location_b
/// );
///
/// let location_unknown = Location::Unknown;
/// //  Otherwise, the first is returned.
/// assert_eq!(
///     variantly::and!(Location::Coord, location_a, location_unknown, (a, b)),
///     location_a
/// );
///
/// // Single-Variable
/// let location_c = Location::ID(1);
/// let location_d = Location::ID(2);
/// assert_eq!(
///     variantly::and!(Location::ID, location_c, location_d, (a)),
///     location_d
/// );
///
/// assert_eq!(
///     variantly::and!(Location::ID, location_c, location_unknown, (a)),
///     location_c
/// );
///
/// // If neither match, the first variant is returned.
/// assert_eq!(
///     variantly::and!(Location::Coord, location_c, location_unknown, (a, b)),
///     location_c
/// );
///
/// ```
#[macro_export]
macro_rules! and {
    ($variant:path, $enum_a:expr, $enum_b:expr, ($($vars:tt),*)) => {
        match (&$enum_a, $enum_b) {
            (&$variant(..), $variant($($vars),*)) => $variant($($vars),*),
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
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
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
/// let temp_a = variantly::and_then!(Temperature::F, temp_a, f_boil, (a));
/// assert_eq!(temp_a, Temperature::F(212));
///
/// let temp_b = Temperature::C(0);
/// let temp_b = variantly::and_then!(Temperature::F, temp_b, f_boil, (a));
/// assert_eq!(temp_b, Temperature::C(0));
/// ```
#[macro_export]
macro_rules! and_then {
    ($variant:path, $enum:expr, $and_then:tt, ($($vars:tt),*)) => {
        match $enum {
            $variant($($vars),*) => {
                let ($($vars),*) = $and_then(($($vars),*));
                $variant($($vars),*)
            },
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
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
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
/// assert_eq!(32, variantly::expect!(Temperature::F, temp, "This should have been in fahrenheit", (a)))
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
/// variantly::expect!(Temperature::F, temp, "This should have been in fahrenheit", (a));
/// ```
#[macro_export]
macro_rules! expect {
    ($variant:path, $enum:expr, $msg:expr, ($($vars:tt),*)) => {
        variantly::unwrap_or_else!($variant, $enum, (|| panic!("{}", $msg)), ($($vars),*))
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
    // TODO, this can be simplified by using the tt pattern applied in other macros.
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
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
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
/// assert_eq!(Some(32), variantly::ok!(Temperature::F, temp, (a)));
/// assert_eq!(None, variantly::ok!(Temperature::C, temp, (a)));
/// ```
#[macro_export]
macro_rules! ok {
    ($variant:path, $enum:expr, ($($vars:tt),*)) => {
        match $enum {
            $variant($($vars),*) => Some(($($vars),*)),
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
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
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
/// assert_eq!(Ok(373), variantly::ok_or!(Temperature::K, temp, err, (a)));
///
/// let temp = Temperature::C(100);
/// assert_eq!(Err(err), variantly::ok_or!(Temperature::K, temp, err, (a)));
/// ```
#[macro_export]
macro_rules! ok_or {
    ($variant:path, $enum:expr, $err:expr, ($($vars:tt),*)) => {
        variantly::ok_or_else!($variant, $enum, (|| $err), ($($vars),*))
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
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
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
/// assert_eq!(Ok(373), variantly::ok_or_else!(Temperature::K, temp, or_else, (a)));
///
/// let temp = Temperature::C(100);
/// assert_eq!(Err(or_else()), variantly::ok_or_else!(Temperature::K, temp, or_else, (a)));
/// ```
#[macro_export]
macro_rules! ok_or_else {
    ($variant:path, $enum:expr, $else:tt, ($($vars:tt),*)) => {
        match $enum {
            $variant($($vars),*) => Ok(($($vars),*)),
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
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
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
///     variantly::or!(Temperature::F, temp_a, temp_b, (a)),
///     temp_a
/// );
///
/// let temp_c = Temperature::C(100);
/// assert_eq!(
///     variantly::or!(Temperature::C, temp_a, temp_c, (a)),
///     temp_c
/// );
/// ```
#[macro_export]
macro_rules! or {
    ($variant:path, $enum_a:expr, $enum_b:expr, ($($vars:tt),*)) => {
        match $enum_a {
            $variant($($vars),*) => $variant($($vars),*),
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
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
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
///     variantly::or_else!(Temperature::F, temp, fallback, (a)),
///     temp
/// );
///
/// let temp = Temperature::C(100);
/// assert_eq!(
///     variantly::or_else!(Temperature::F, temp, fallback, (a)),
///     Temperature::F(0)
/// );
/// ```
#[macro_export]
macro_rules! or_else {
    ($variant:path, $enum:expr, $or_else:tt, ($($vars:tt),*)) => {
        match $enum {
            $variant($($vars),*) => $variant($($vars),*),
            _ => {
                let ($($vars),*) = $or_else();
                $variant($($vars),*)
            },
        }
    };
}

/// Return the value contained by the enum instance if it is of the expected variant, otherwise panics.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
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
/// assert_eq!(32, variantly::unwrap!(Temperature::F, temp, (a)))
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
/// variantly::unwrap!(Temperature::C, temp, (a));
///
/// ```
#[macro_export]
macro_rules! unwrap {
    ($variant:path, $enum:expr, ($($vars:tt),*)) => {
        variantly::unwrap_or_else!($variant, $enum, (|| panic!()), ($($vars),*))
    };
}

/// Return the value contained by the enum instance if it is of the expected variant, otherwise returns a passed in default value.
///
/// # Arguments
///
/// * `variant` - An enum variant
/// * `enum` - An instance of an enum of the same base type as `variant`
/// * `or` - The value to return if `enum` is not of the expected variant
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
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
/// assert_eq!(32, variantly::unwrap_or!(Temperature::F, temp, 100, (a)));
/// assert_eq!(100, variantly::unwrap_or!(Temperature::C, temp, 100, (a)));
///
/// ```
#[macro_export]
macro_rules! unwrap_or {
    ($variant:path, $enum:expr, $or:expr, ($($vars:tt),*)) => {
        variantly::unwrap_or_else!($variant, $enum, (|| $or), ($($vars),*))
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
/// * `vars` - Raw tokens to represent any contained variables in the enum variant.
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
/// assert_eq!(32, variantly::unwrap_or_else!(Temperature::F, temp, expensive_calculation, (a)));
/// assert_eq!(100, variantly::unwrap_or_else!(Temperature::C, temp, expensive_calculation, (a)));
///
/// ```
#[macro_export]
macro_rules! unwrap_or_else {
    ($variant:path, $enum:expr, $else:tt, ($($vars:tt),*)) => {
        match $enum {
            $variant($($vars),*) => ($($vars),*),
            _ => $else(),
        }
    };
}
