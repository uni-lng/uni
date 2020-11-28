# justc1

Stage 1 compiler of *Just*.

This compiler is self-contained.
It does not have any external dependency.

## Responsibilities

This is used to build the core packages for `justc`:

- `just_compiler_core`
- `just_fs`
- `justc`

This compiler only support language features needed to build those packages.
The limitations include:

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
- (maybe) no structural type
  - no type reduction
- (maybe) no incremental build
  - no memory balancing (useful only to language service)
