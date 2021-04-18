# Design Choices

## No Implicit Return

*Rust* support this:

```rs
fn foo() {
  let y = 1;
  y
}
```

The last statement without semi-colon means it is the result value.

It is equivalent to:

```rs
fn foo() {
  let y = 1;
  return y;
}
```

*Just* will not do support this.
The `return` keyword is required.

This is because focus on tooling and want to provide code completion and error indication when user is typing the code.

If supporting this feature, it means every line typed before it is completed, can be the return value,
this will generate incorrect completion and error messages,
degrading development experience.
