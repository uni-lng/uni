# just_compiler

`just_compiler` is the initial *Just* compiler written in `Rust`.

It is used to bootstrap the first compiler written in *Just*.

It supports the minimum syntax needed to create the first compiler.

- no support to compile a single file
- only support fixed entry points
  - `main.just`
  - `lib.just`
- no cross-platform support. Only compiles to x86
- no optimization
- minimal compiler options
- no dependency injection
  - can be avoided as we will not build tests
- no structural type

## Usage

```sh
# justc <folder>
justc just/just_std_io
justc just/just_std_fs
justc just/just_compiler_build_tools
justc just/just_compiler
```

## Responsibilities

- build the basic components for the first *just* compiler written in *just*.

## Process

- 🚧 lexer: source code string ▶️ `Token` stream.
- ⌛️ parser: `LexerToken` stream ▶️ `AST`.
- ⌛️ binder: `AST` ▶️ `Symbols`
- ⌛️ transformer(s): `AST` ⏩ other IRs (Intermediate Representations)
- ⌛️ checker(s): any IRs ▶️ Syntax and Semantic Validations
- ⌛️ emitter: multiple IRs ▶️ LLVM IR

## References

- Rust Compiler: <https://rustc-dev-guide.rust-lang.org/overview.html>
- AssemblyScript Compiler: <https://github.com/AssemblyScript/assemblyscript/tree/master/src>
- TypeScript Compiler:
  - <https://github.com/microsoft/TypeScript/wiki/Architectural-Overview>
  - <https://github.com/microsoft/TypeScript/wiki/Compiler-Internals>
  - <https://basarat.gitbook.io/typescript/overview>
