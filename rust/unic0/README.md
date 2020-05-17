# unic0

`unic0` is a stage 0 Uni compiler written in Rust.

It's sole purpose is to compile `unic1`: stage 1 Uni compiler written in Uni.

It targets only x86.
Cross compilation is handled by `unic`, the actual compiler.

```sh
# unic0 <folder>
unic0 uni/unic1
```

It will produce `unic1` as `uni/unic1/bin/unic1`

## Responsiblities

- parse user input when user executes `unic`.
- produce a partial `CompilerOptions` to be used in the `...`.
- provide external dependencies such as `FileSystem` and `UI` to the application.
