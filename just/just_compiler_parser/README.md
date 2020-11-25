# just_parser

Just Language Parser.

The parser will generate a `SourceRepresentation` from the source code.

`SourceRepresentation` contains meta data about the source code including AST.

It can be partially updated, e.g when the source is changed in an editor.

## Features

```rust
pub struct SourceRepresentation {
  node: SourceNode
}

pub fn parse(source: &str) -> SourceRepresentation;

pub fn update(this: &SourceRepresentation, slice: &str, index: u32) -> SourceDelta
```
