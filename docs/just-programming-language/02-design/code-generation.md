# Code Generation

There are three ways to generate code:

## Full-fledge Code Generator

Full-fledge code generator takes some complex code templates,
and generate code based on those templates.

The results are typically file(s) and folder(s) or code.

## Code Snippets

Code snippets can take some simple templates,
and generate code based on those templates.

The results are typically one single file or some code chunks.

*Just* places more focus here,
and implement some *rust* macro features (e.g. `println!`) as code snippets.

## Macros

Macros typically are recognized language syntax,
that generates resulting code during compile time.

They are typically useful to generate code that takes a while to write and require complex input from surrounding code.
