# unic0

`unic0` is a stage 0 Uni compiler written in Rust.

It is used to build:

- stage 1 Uni compiler from the source code in `uni\unic`.
- build tools necessary to build the compiler.

In order to spend more time dogfooding in Uni directly,
some design and language features are skipped:

- no support to compile a single file
- only support fixed entry points
  - `main.uni`
  - `lib.uni`
- no cross-platform support. Only compiles to x86
- no optimization
- ignore comments
- minimal compiler options
  - not sure if this fit into this list
- no dependency injection
  - can be avoided as we will not build tests
- (maybe) no structural type
  - no type reduction
- (maybe) no incremental build
  - no memory balancing (useful only to language service)

## Usage

```sh
# unic0 <folder>
unic0 uni/unic1
```

It will produce `unic1` as `uni/unic1/bin/unic1`

## Responsibilities

- parse user input when user executes `unic0`.
- produce a partial `CompilerOptions` to be used in the `...`.
- provide external dependencies such as `FileSystem` and `UI` to the application.
