# Data Types

There are two kinds of data types in *just*: **value types** and **reference types**.

Like most other languages,
**value types** hold the data within their own memory allocation,
and they are stored directly in the stack during function calls,
while **reference types** hold a pointer which points to the actual data stored in heap.

*just* has three classes of value types:

- **singular types**: a class of types that represent a single **native** value.
- **literal types**: a class of types that represent a specific **literal**.
- **collection types**: a class of types that represent a fixed-size collection of values.

## Singular Types

*just* has four kinds of singular types: boolean, integers, floating-points, and character.

### Boolean Type

The most basic singular type is the simple `true` / `false` value.

It is specified as `bool`.

```just file=./bool.just
let t = true;
let f: bool = false;
```

### Integer Types

An integer is a number without a fractional component.
It can be signed or unsigned, and take up specific amount of space.

| Space   | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| varies  | isize  | usize    |

Signed integers are stored using two's complement representation.

The size of `isize` and `usize` depends on the binary is built for 32-bit or 64-bit architecture.

You can write integer literals in multiple ways.

| Literals | Example     |
| -------- | ----------- |
| Decimal  | 98_222      |
| Hex      | 0xff        |
| Octal    | 0o77        |
| Binary   | 0b1111_0000 |
| Byte     | b'A'        |

Decimal representations are typed as `i32` by default.

Hex, octal, and binary representations are typed as `u32` by default.

Byte representations are always `u8`.

Decimal, hex, octal, and binary type are automatically expended if the literal cannot be fit in 32 bits.
Also, they can be specified using algebra:

```just file=./int.just
let mut mb = 8 * 1024 * 1024
let mut gb = 2^33
```

Since specifying integer types is very common in systems programming,
you can specify the type by adding a type suffix to the literal:

```just file=./uint.just
let mut x = 57u64  // `x: u64`
let mut y = 0x01u8 // `y: u8`
```

You can also use `_` as a visual separator, such as `1_000`.

### Floating-Point Types

A floating-point is a number with decimal points.
*Just* has two floating-point types: `f32` and `f64`.

Floating-point numbers are represented according to the [IEEE-754 standard](https://en.wikipedia.org/wiki/IEEE_754).
The `f32` type is a single-precision float, and `f64` has double precision.

The same syntax for integer applies to floating-point types:

```just file=./float.just
let mut f1 = 1_000.123_456
let mut f2 = 0.1f64
let mut f3 = -1.23e-8_f64
let mut f4 = 0.1
let mut f5 = 0.0
let mut f6 = 2.
```

### Character Type

*Just* `char` is a four bytes Unicode Scalar Value.
Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive.

Like most other languages, `char` is enclosed within single-quotes.

## Literal Types

A literal refers to a value that can be represented in the code without additional processing.
Boolean, integer, floating-point, character, and string are example of literals.

Literal types refer to the concrete type of a specific literal.

```just file=./literal.just
let t: true = true
let n: 1 = 1
let c: 'a' = 'a'
let s: "foo" = "foo"
```

The literal types is a sub-type of their respective types.
e.g. `true` is a sub-type of `bool`, and `3` is a sub-type of `u8`, `i8`, `u16`, `i16`, and so on.

Since `let x = <literal>` is immutable,
the narrower literal type is used as the default.

Since `let mut x = <literal>` is mutable,
the wider type is used as the default.

```just file=./literal-let-vs-mut.just
let a = 1 // `a: 1`
let mul b = 1 // `b: i32`
```

For similar reason,
function params with default uses the wider type.

```just file=./literal-param-with-default.just
let foo = (x = 1 /* `x: i32` */) => {
  /* -snip- */
}
```

You can override this behavior by type annotation.

## Collection Types

Collection types are data types that groups multiple values into a single type.

Since they are value types, their data are stored directly in the stack during function calls.

This make them more suitable in passing data between function calls when the data is small.

However, if the data is large or consist of a lot of elements,
it will negatively impact performance.

*Just* has two collection types: `array` and `tuple`.

### Array Type

The `array` type is a fixed-size, homogeneous collection.

This means every element of an array must have the same type.

```just file=./array.just
let a: [i32; 3] = [1, 2, 3];
```

`Just` will infer a variable to `array` type if they satisfy these criteria.

```just
let a = [1, 2, 3, 4, 5] // `a: [i32; 5]`
```

You can also initialize an array with the same value in every element:

```just
let a = [3; 5]; // same as [3, 3, 3, 3, 3]
```

You can access array element using indexing:

```just
let a = [1, 2, 3];

let first = a[0];
```

### Tuple Type

The `tuple` type is a fixed-size, heterogeneous collection.

This mean some elements in the collection has different types than others.

The way to initialize a `tuple` is the same as `array`:

```just file=./tuple.just
let t = [1, 2, 'a']; // `t: [i32, i32, char]`
```

You can specify the type of each element explicitly:

```just file=./tuple-explicit.just
let t: [i32, u32, char] = [1, 2, 'a'];
```

`tuple` also support the same `array` type syntax:

```just file=./tuple-with-array.just
let t: [char, i32; 3, bool] = ['a', 1, 2, 3, true];
```

## Reference Types

**Reference types** contain a pointer to another memory location that holds the data.

*Just* has three kinds of reference types: `function`, `struct`, and `slice`.

### Function Type

`Function` is a fundamental type in any programming language.

In *just*, `function` is a first-class type like other value and reference types.

A `function` is defined using the fat-arrow syntax:

```just file=./function.just
let foo = () => "foo"
let incIfOdd = (v: i32) => {
  if (v % 2) {
    v
  } else {
    v + 1
  }
}
```

The last expression will be used as the return value.
If the function will not return anything, use `;` to end the expression:

```just file=./function-no-return.just
let echo = (v: bool) => io.write(v);
```

Note that `;` is applied to the function body in the above example, not the `let` statement.

The return type of the function is inferred automatically.
You can annotate the return type explicitly using the thin-arrow syntax:

```just file=./function-explicit-return-type.just
let inc = (v: i32) -> i32 => v + 1
```

When a `function` is defined, a `closure` is created to capture any environment it tries to access.
The data and references captured in the `closure` is stored as the `context` of the `function`.

We will talk about `closure` in more detail in the future.

### Struct Type

### Slice Type
