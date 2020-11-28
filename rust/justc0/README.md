# unic0

`justc0` is a stage 0 *Just* compiler written in `Rust`.

It is used to build `justc1`, the stage 1 *Just* compiler.

In order to spend more time dogfooding in *Just* directly,
some design and language features are skipped:

- no support to compile a single file
- only support fixed entry points
  - `main.just`
  - `lib.just`
- no cross-platform support. Only compiles to x86
- no optimization
- ignore comments
- minimal compiler options
- no dependency injection
  - can be avoided as we will not build tests
- no structural type

## Usage

```sh
# justc0 <folder>
justc0 just/justc1
```

It will produce `justc1` in `just/justc1/bin/justc1`

## Responsibilities

- parse user input when user executes `justc1`.
- produce a partial `CompilerOptions` to be used in the `...`.
- provide external dependencies such as `FileSystem` and `UI` to the application.
