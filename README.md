# Variantly
Derive helper methods for enum variants that are familiar from `Option` & `Result` such as `unwrap_or` or `and_then`.
# Example
```rust
#[derive(variantly::Variantly)]
enum Color {
    RGB(u8, u8, u8),
    HSV(u8, u8, u8),
    Grey(u8),
    FromOutOfSpace,
    #[variantly(rename = "darkness")]
    Black,
}

fn example() {
    let color = Color::HSV(123, 45, 67);

    // boolean helper method for determining variant:
    assert!(color.is_hsv());
    assert!(!color.is_rgb());

    // Get inner values:
    let (h, s, v) = color.unwrap_hsv();
    assert_eq!((h, s, v), (123, 45, 67));

    // Single values don't require tuple destructuring:
    let color = Color::Grey(128);
    let value = color.unwrap_grey();
    assert_eq!(value, 128);

    // Alter inner value, only if hsv:
    let color = Color::HSV(111, 22, 33);
    let color = color.and_then_hsv(|(h, s, _)| (h, s, 100));
    assert_eq!(color.unwrap_hsv(), (111, 22, 100));

    // Safely unwrap with a fallback:
    let color = Color::RGB(255, 255, 0);
    let (r, g, b) = color.unwrap_or_rgb((0, 0, 0));
    assert_eq!((r, g, b), (255, 255, 0));
    // Since color is of the HSV variant, the default is not used.

    // Safely unwrap using the fallback
    let color = Color::FromOutOfSpace;
    let (r, g, b) = color.unwrap_or_rgb((0, 0, 0));
    assert_eq!((r, g, b), (0, 0, 0));

    // Convert into an Option
    let color = Color::RGB(0, 255, 255);
    let optional_rgb = color.rgb();
    assert_eq!(Some((0, 255, 255)), optional_rgb);

    // Convert into a Result
    let color = Color::RGB(255, 0, 255);
    let result_rgb = color.rgb_or("Error: This is not an RGB variant!");
    assert_eq!(Ok((255, 0, 255)), result_rgb);

    // Operations like this can also use their familiar `_else` versions:
    let color = Color::FromOutOfSpace;
    let result_rgb = color.rgb_or_else(|| Some("This is a computationally expensive error!"));
    assert!(result_rgb.is_err());

    // The `#[variantly(rename = "darkness")]` attribute renames derived methods:
    let color = Color::Black;
    assert!(color.is_darkness())
}
```
# Derived Methods
In the naming of all methods described here, replace the `{variant_name}` with the snake_case formatted name of the given variant.

## Option & Result Conversion
Use the below methods to convert the enum into either an option or result:

### `pub fn {variant_name}(self) -> Option(...)`
If the enum is of the given variant, returns a `Some` containing the inner variant value. Otherwise, return None.

#### Example
```rust
let color = Color::HSV(1,2,3);

let option = color.hsv();
assert_eq!(Some((1, 2, 3)), option);

let color = Color::FromOutOfSpace;
assert_eq!(None, color.rgb());
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

### `pub fn {variant_name}_ref(&self) -> Option(&...)`
If the enum is of the given variant, returns a `Some` containing a ref to the inner variant value. Otherwise, return None.

#### Example
```rust
let color = Color::HSV(1,2,3);

let option = color.hsv_ref();
assert_eq!(Some((&1, &2, &3)), option);

let color = Color::FromOutOfSpace;
assert_eq!(None, color.rgb_ref());
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

### `pub fn {variant_name}_mut(&mut self) -> Option(&mut...)`
If the enum is of the given variant, returns a `Some` containing a mutable ref to the inner variant value. Otherwise, return None.

#### Example
```rust
let mut color = Color::HSV(1,2,3);

let option = color.hsv_mut();
assert_eq!(Some((&mut 1, &mut 2, &mut 3)), option);

let mut color = Color::FromOutOfSpace;
assert_eq!(None, color.rgb_mut());
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

### `pub fn {variant_name}_or<E>(self, err: E) -> Result<(...), E>`
If the enum is of the given variant, returns a `Result::Ok` containing the inner value. Otherwise, return `Result::Err` containing `err`.

#### Example
```rust
let color = Color::HSV(1,2,3);

let result = color.hsv_or("Error: Not an HSV!");
assert_eq!(Ok((1, 2, 3)), result);

let color = Color::FromOutOfSpace;
let result = color.hsv_or("Error: Not an HSV!");
assert_eq!(Err("Error: Not an HSV!"), result);
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

### `pub fn {variant_name}_ref_or<E>(&self, err: E) -> Result<(&...), E>`
If the enum is of the given variant, returns a `Result::Ok` containing a ref to the inner value. Otherwise, return `Result::Err` containing `err`.

#### Example
```rust
let color = Color::HSV(1,2,3);

let result = color.hsv_ref_or("Error: Not an HSV!");
assert_eq!(Ok((&1, &2, &3)), result);

let color = Color::FromOutOfSpace;
let result = color.hsv_ref_or("Error: Not an HSV!");
assert_eq!(Err("Error: Not an HSV!"), result);
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

### `pub fn {variant_name}_mut_or<E>(&mut self, err: E) -> Result<(&mut...), E>`
If the enum is of the given variant, returns a `Result::Ok` containing a mutable ref to the inner value. Otherwise, return `Result::Err` containing `err`.

#### Example
```rust
let mut color = Color::HSV(1,2,3);

let result = color.hsv_mut_or("Error: Not an HSV!");
assert_eq!(Ok((&mut 1, &mut 2, &mut 3)), result);

let mut color = Color::FromOutOfSpace;
let result = color.hsv_mut_or("Error: Not an HSV!");
assert_eq!(Err("Error: Not an HSV!"), result);
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

### `pub fn {variant_name}_or_else<E, F: FnOnce() -> E>(self, f: F) -> Result<(...), E>`
If the enum is of the given variant, returns a `Result::Ok` containing the inner variant value. Otherwise, calls `f` to calculate a `Result::Err`.

#### Example
```rust
let color = Color::HSV(1,2,3);

let result = color.hsv_or_else(|| "This is an expensive error to create.");
assert_eq!(Ok((1, 2, 3)), result);

let color = Color::FromOutOfSpace;
let result = color.hsv_or_else(|| "This is an expensive error to create.");
assert_eq!(Err("This is an expensive error to create."), result);
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

### `pub fn {variant_name}_ref_or_else<E, F: FnOnce() -> E>(&self, f: F) -> Result<(&...), E>`
If the enum is of the given variant, returns a `Result::Ok` containing a ref to the inner variant value. Otherwise, calls `f` to calculate a `Result::Err`.

#### Example
```rust
let color = Color::HSV(1,2,3);

let result = color.hsv_ref_or_else(|| "This is an expensive error to create.");
assert_eq!(Ok((&1, &2, &3)), result);

let color = Color::FromOutOfSpace;
let result = color.hsv_ref_or_else(|| "This is an expensive error to create.");
assert_eq!(Err("This is an expensive error to create."), result);
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

### `pub fn {variant_name}_mut_or_else<E, F: FnOnce() -> E>(&mut self, f: F) -> Result<(&mut...), E>`
If the enum is of the given variant, returns a `Result::Ok` containing a mut ref to the inner variant value. Otherwise, calls `f` to calculate a `Result::Err`.

#### Example
```rust
let mut color = Color::HSV(1,2,3);

let result = color.hsv_mut_or_else(|| "This is an expensive error to create.");
assert_eq!(Ok((&mut 1, &mut 2, &mut 3)), result);

let mut color = Color::FromOutOfSpace;
let result = color.hsv_mut_or_else(|| "This is an expensive error to create.");
assert_eq!(Err("This is an expensive error to create."), result);
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

## Accessing Inner Values
Use the below methods to easily access the inner value of a given variant.

## `pub fn expect_{variant_name}(self, msg: &str) -> (...)`
Returns the contained value.

### Panics
Panics if the enum is not of the given variant with the custom message `msg`.

### Example
```rust
#[derive(variantly::Variantly)]
enum Color {
    HSV(u8, u8, u8),
    Grey(u8),
}

let color_a = Color::HSV(1,2,3);
let color_b = Color::Grey(10);

let (h, s, v) = color_a.expect_hsv("This should be an hsv");
assert_eq!((h, s, v), (1, 2, 3));

let grey = color_b.expect_grey("This should be grey");
assert_eq!(grey, 10);
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

## `pub fn unwrap_{variant_name}(self) -> (...)`
Returns the contained value.

### Panics
Panics if the enum is not of the given variant.

### Example
```rust
let color_a = Color::HSV(1,2,3);
let color_b = Color::Grey(10);

let (h, s, v) = color_a.unwrap_hsv();
assert_eq!((h, s, v), (1, 2, 3));

let grey = color_b.unwrap_grey();
assert_eq!(grey, 10);
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

## `pub fn unwrap_or_{variant_name}(self, fallback: (...)) -> (...)`
Returns the contained value if the enum is of the given variant, otherwise returns the provided `fallback`.

### Example
```rust
let color_a = Color::HSV(1,2,3);
let color_b = Color::Grey(10);

let (h, s, v) = color_a.unwrap_or_hsv((4, 5, 6));
assert_eq!((h, s, v), (1, 2, 3));

let color = color_b.unwrap_or_rgb((4, 5, 6));
assert_eq!(color, (4, 5, 6));
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

## `pub fn unwrap_or_else_{variant_name}<F: FnOnce() -> (...)>(self, f: F) -> (...)`
Returns the contained value if the enum is of the given variant, otherwise computes a fallback from `f`.

### Example
```rust
let color_a = Color::HSV(1,2,3);
let color_b = Color::Grey(10);

let (h, s, v) = color_a.unwrap_or_else_hsv(|| (4,5,6));
assert_eq!((h, s, v), (1, 2, 3));

let (h, s, v) = color_b.unwrap_or_else_hsv(|| (4,5,6));
assert_eq!((h, s, v), (4, 5, 6));
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

## Testing Variant Type
Use the below methods to test whether a variant is of the given type.

### `pub fn is_{variant_name}(self) -> bool`
Returns `true` if the enum is of the given variant.

#### Example
```rust
let color = Color::FromOutOfSpace;
assert!(color.is_from_out_of_space());
```

*Note: Available for all variant types*

### `pub fn is_not_{variant_name}(self) -> bool`
Returns `true` if the enum is *not* of the given variant.

#### Example
```rust
let color = Color::HSV(1,2,3);
assert!(color.is_not_rgb());
```

*Note: Available for all variant types*

## Compare & Process Specific Variant
Use the below to process and compare a specific enum variant.

### `pub fn and_{variant_name}(self, enum_b: GivenEnum) -> GivenEnum`
Returns `enum_b` if both self and `enum_b` are of the given variant. Otherwise returns `self`.

#### Example
```rust
let color_a = Color::HSV(1,2,3);
let color_b = Color::HSV(4,5,6);
let and = color_a.and_hsv(color_b);
assert_eq!(
    and,
    Color::HSV(4,5,6),
);
```

*Available for all variant types*

### `pub fn and_then_{variant_name}<F: FnOnce((...)) -> (...)>(self, f: F) -> Self`
Returns the enum as is if it is not of the given variant, otherwise calls `f` with the wrapped value and returns the result.

#### Example
```rust
let color_a = Color::HSV(1,2,3);

let and = color_a.and_then_hsv(|(h, s, _)| (h, s, 4));
assert_eq!(
    and,
    Color::HSV(1, 2, 4),
);
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

### `pub fn or_{variant_name}(self, enum_b: GivenEnum) -> GivenEnum`
Returns `self` if it is of the given variant, otherwise returns `enum_b`.

#### Example
```rust
let color_a = Color::HSV(1,2,3);
let color_b = Color::RGB(4,5,6);
let or = color_a.or_rgb(color_b);
assert_eq!(
    or,
    Color::RGB(4,5,6),
);
```

*Available for all variant types*

### `pub fn or_else_{variant_name}<F: FnOnce() -> (...)>(self, f: F) -> Self {`
Returns `self` if it is of the given variant, otherwise calls `f` and returns the result.

#### Example
```rust
let color = Color::HSV(1,2,3);
let color = color.or_else_rgb(|| (4,5,6));
assert_eq!(
    color,
    Color::RGB(4,5,6),
);
```

*Note: Available only for tuple-style variants such as Color::RGB(200, 40, 180), or Color::Grey(10)*

# Renaming Methods
The `variantly` attribute may be placed on a variant in order to customize the resulting method names. The value set against `rename` inside the attribute will be used in place of the snake_cased variant name when constructing derived method names.
```rust
#[derive(variantly::Variantly)]
enum SomeEnum {
    #[variantly(rename = "variant_a")]
    SomeVariantWithALongName(String),
    VariantB,
}

let variant = SomeEnum::SomeVariantWithALongName(String::from("Hello"));
assert!(variant.is_variant_a());
```
Methods associated with `SomeVariantWithALongName` will now be accessible only with the `variant_a`
suffix, such as `.unwrap_or_else_variant_a()`. This can help control overly verbose fn names.
Note that the input to `rename` is used as is and is not coerced into snake_case.

The above is also relevant when two variant names would expand to create conflicting method names:
```rust
#[derive(variantly::Variantly)]
enum SomeEnum {
    #[variantly(rename = "capital")]
    ABC,
    #[variantly(rename = "lower")]
    abc,
}
```
Without the `rename` attribute in the above, both variants would create conflicting functions such as `.is_abc()` due to the coercion to snake_case.
This is avoided by using the `rename` input to create meaningful and unique fn names.

#### License

<sup>
Licensed under <a href="LICENSE">MIT license</a>.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate shall be licensed as above, without any additional terms or conditions.
</sub>