# Data Types

`just` has three classes of data types:

- **value types**: a class of types that represent a single **native** value.
- **reference types**: a class of types that represent a **complex** value.
- **literal types**: a class of types that represent a specific literal.

## Value Types

`just` has four kinds of value types: boolean, integers, floating-points, and character.

These types hold the data within their own memory allocation. i.e. they are stored in the stack.

Other value types to consider:

- `v128`:
  - [in AssemblyScript](https://www.assemblyscript.org/types.html#types)
  - [SIMD](https://en.wikipedia.org/wiki/SIMD)

### Boolean Type

The most basic value type is the simple `true` / `false` value.

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
| varies  | int    | uint     |

Signed integers are stored using two's complement representation.

The size of `int` and `uint` depends on the binary is built for 32-bit or 64-bit architecture.

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

```just
let mb = 8 * 1024 * 1024
let gb = 2^33
```

Since specifying integer types is very common in systems-level programming,
you can specify the type by adding a type suffix to the literal:

```just
let x = 57u64  // `x: u64`
let y = 0x01u8 // `y: u8`
```

You can also use `_` as a visual separator, such as `1_000`.

### Floating-Point Types

A floating-point is a number with decimal points.
`Just` has two floating-point types: `f32` and `f64`.

Floating-point numbers are represented according to the [IEEE-754 standard](https://en.wikipedia.org/wiki/IEEE_754).
The `f32` type is a single-precision float, and `f64` has double precision.

The same syntax for integer applies to floating-point types:

```just
let f1 = 1_000.123_456
let f2 = 0.1f64
let f3 = -1.23e-8f64
let f4 = 0.1
let f5 = 0.0
let f6 = 2.
```

### Character Type

`Just` `char` is a four bytes Unicode Scalar Value.
Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.

## Reference Type

## Literal Types