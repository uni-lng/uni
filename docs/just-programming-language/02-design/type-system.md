# Type System

## Static vs Dynamic

`static` and `dynamic` is a very common way to categorize programming languages into.
For example, `C`, `C++`, `C#`, and `Java` are statically typed,
while `JavaScript`, `Python`, and `PHP` are dynamically typed.

However, if you do a quick search you will find that there are still some confusion of what they actually mean or the definition is not useful to classify modern programming languages:

- <https://en.wikipedia.org/wiki/Type_system#Type_checking>
- <https://docs.oracle.com/cd/E57471_01/bigData.100/extensions_bdd/src/cext_transform_typing.html>
- <https://www.sitepoint.com/typing-versus-dynamic-typing/>
- <https://hackernoon.com/i-finally-understand-static-vs-dynamic-typing-and-you-will-too-ad0c2bd0acc7>
- <https://stackoverflow.com/questions/1517582/what-is-the-difference-between-statically-typed-and-dynamically-typed-languages>

In order to have a meaningful description of the type system in *just* with respect to `static` and `dynamic` typing,
it is important to make it clear about what they really are and where they fall short.

Here is a short description for all terms involved:

| Term                  | Description                                                                   |
| --------------------- | ----------------------------------------------------------------------------- |
| Identifier            | Tokens which name the language entities.                                      |
|                       | i.e. name of variable, parameter, function, package, etc.                     |
| Run time              | When the program is executed.                                                 |
| Runtime               | The time and space where the program is executing on.                         |
| Compile Time          | When the source code of a program is being compiled.                          |
|                       | It loosely include other processes that need to analyze the source code       |
|                       | in some form such as linting and validation on editors.                       |
| Static Typing         | The ability to **annotate** the type of an **identifier** in the source code. |
|                       | a.k.a. **Type Annotation**                                                    |
| Static Type           | Type that are defined explicitly in source code                               |
|                       | and can be resolved at **compile time**.                                      |
| Dynamic Type          | Type that can only be resolved at **run time**.                               |
| Inferred Type         | Type that are inferred at **compile time**.                                   |
| Type Inference        | The ability to (and the process of) inferring types at **compile time**.      |
| Static Type Checking  | **Compile time** type checking to verify the type safety of a program.        |
| Dynamic Type Checking | **Runtime** type checking to differentiate types.                             |

Given these terms, we can define:

**Statically Typed Programming Language**, or **Static Typed Language** in short,
is a programming language that has **static typing** on all **identifier**s.

**Dynamically Typed Programming Language**, or **Dynamic Typed Language** in short,
is a programming language that lacks **static typing** on some **identifier**s.

These definitions fits well into our typical understanding of static and dynamic typed language,
at the same time it provides the necessary premise to determine if a language is statically or dynamically typed.

For example, under these definitions,
PHP 7 is still a dynamically typed language.
While you can annotate parameter type:

```php
function foo(string $value) {
  // -snip-
}
```

You cannot annotate **variable** type:

```php
string $abc; // this is not valid PHP
```

Note that the definitions do not mention **static type checking**.
This is because **static type checking** is a tool that process the type information at **compile time**.

Imagine that a programming language that only work on primitive types.
In such language, you don't need **static typing**,
but you can still perform **static type checking** at compile time.

In the same sense,
you can create a code analyzer for any dynamically typed language,
process the AST and create additional type information by **type inference**,
and generate error when a variable has changed type during execution.

If you enforce the usage of such analyzer,
does the language becomes a statically typed language?

The key take away is that the language is static or dynamic is depend on how the code can be written,
not how the code can be processed.

*just* is a **statically typed language** with **dynamic type** support.

**Type annotation**  is optional as long as the type can be inferred.

This mean the following are valid:

```just
let x = true // `x: bool`

let count = (value: []) -> int => { /* -snip- */ }

let c = count([]) // `c: int`
```

**Type union** allows the variable to be declared with multiple types.
The variable can be used as long as the operation is applicable to all types in the union,
or when the type is narrowed or asserted for the operation.

```just
let mut x: number | string = 1

console.log(x) // accept any type

x.length // invalid. `length` only applicable to `string`

if (typeof x === 'string') {
  x.length // valid. `x` is narrowed to `string`
}
```

**Dynamic type** is required to support **type union**.

## Structural vs Nominal

*Just* supports both `nominal` and `structural` type out of the box.
All types are `structural` by default.

To declare a `nominal` type, use the `nominal` decorator:

```just
@nominal
pub let copy = () => {
  // -snip-
}
```
