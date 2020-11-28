# TypeScript

|                                      |                                                         |
| ------------------------------------ | ------------------------------------------------------- |
| 👍 gradual type                      | allows variable type to be inferred or dynamic          |
| 👍 structural type                   | type equivalency is determined by shape instead of name |
| 👎 unsound type                      | there are holes and pitfalls in the type system         |
| 👎 incomplete type-level programming | type-level programming lack some basic functionalities  |

## Structural Type

`structual` type is superior than `nominal` type in many ways.

*Just* supports both `nominal` and `structural` type out of the box.
All types are `structural` by default.

## Unsound type

Producing a sound type system is a specific non-goal of TypeScript.

It is very difficult to create a sounded type system in TypeScript,
and even if it can, the resulting type system can be very tedious to use.

The major limitation here is because TypeScript is based on JavaScript.
And you can do a lot of crazy thing in JavaScript.

TypeScript prioritizes in describing JavaScript program and rightly so.

So unfortunately, the type system in TypeScript will remain unsound for a long time.

## Incomplete type-level programming

For similar reasons as [unsound type](#unsound-type),
Type-level programming in TypeScript is improving but still missing some functionalities.
for example, type algebra, and delayed generic evaluation.
