# Compilation Process

## Process

- ⌛️ lexer: source code string ▶️ `Token` stream.
- ⌛️ parser: `LexerToken` stream ▶️ `AST`.
- ⌛️ binder: `AST` ▶️ `Symbols`
- ⌛️ typeInterpreter: `AST` + `Symbols` ▶️ `TypeFacts`
- ⌛️ typeResolver: `TypeQuery` + `TypeFacts` ▶️ `TypeFact`
- ⌛️ other transformer(s): `AST` ⏩ other IRs
- ⌛️ checker(s): any IRs ▶️ Syntax and Semantic Validations
- ⌛️ emitter: multiple IRs ▶️ binary
