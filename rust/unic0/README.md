# unic0

`unic0` is a stage 0 Uni compiler written in Rust.

It's sole purpose is to compile `unic1`: stage 1 Uni compiler written in Uni.

## Responsiblities

- parse user input when user executes `unic0`.
- produce a partial `CompilerOptions` to be used in the `...`.
- provide external dependencies such as `FileSystem` and `UI` to the application.
