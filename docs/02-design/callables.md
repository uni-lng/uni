# Callables

Every programming language provides some ways for the programer to perform some kind of actions.

In Uni, we name the mechanism to perform these actions as **callable**.

You maybe already familiar with two kinds of **callable** in other programing languages: functions and class methods.

There are other kinds of **callables**: macros, getter/setting, implicit type cast, magic methods, etc.

In fact, class methods is nothing more than a function that works on a particular set of data.

That's why Uni only have one kind of **callable**: **function**.

Rust and Go takes a similar approach, as well as functional languages such as Haskell and Lisp variants.

To define a **function**, you use the **fat arrow** syntax:

```uni
([<ctx>|]<params>) => <statement>
```



[1, 'a'].forEach(x => x) // x: int | char
const plusOne = &ctx => ctx.x + 1 // ctx: inferred, automatical reference resolution

Note that unlike the **fat arrow** syntax in JavaScript,
the **fat arrow** syntax in Uni does not capture the enclosing scope.

```uni
({ v }| x) => v + x
((ctx, x) => ctx.v + x)({ v })

({ v } = { v }, x) => v + x
```
