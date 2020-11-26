# Primitive Types

## The Boolean Type

The Boolean type in `just` is specified as `bool`.
The values are `true` and `false`.

```just file=./boolean.just
let t = true;
let f: bool = false;
```

## Number Literals

Numbers are inferred as `int`, `uint` or `float` by default.

`int`:

```just
123
-123
```

`uint`:

```just
0xff // hex
0o70 // oct
0b1111 // binary
```

`float`:

```just
123.0
0.1
0.0
2.
12E+99
```
