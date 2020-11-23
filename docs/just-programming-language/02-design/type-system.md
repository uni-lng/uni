# Type System

`Just` supports both `nominal` and `structural` type out of the box.
All types are `structural` by default.

To declare a `nominal` type, use the `nominal` decorator:

```just
@nominal
pub fn copy() {
  // -snip-
}
```
