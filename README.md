# uni

Uni is an experimental language.

I tries to combine the best parts of the languages I used over the years.
These languages include:

- TypeScript
- JavaScript
- Rust
- Go
- C#

## Design Goals

If I have to summarize Uni with minimum words,
it would be "A structural typed Rust".

Rust gets a lot of things right.
Its ownership concept makes so much sense that makes you wonder why other languages have garbage collection.
It also have a very powerful macro allows you to make the language much easier to write.

But nominal type is its most glaring shortcoming IMO.
It makes the code unnecessary cumbersome and creates difficult roadblocks when it comes to design and architecture.

Uni is targeted to be a system level language just like Rust,
but it also focus of tooling support and syntax purity.

In general, Uni has the following characteristics:

- statically type with strong type inference
  - type inference is the solution to the "static vs dynamic" debate.
  - Uni aims to only require type declaration at function parameters.\
    Everything else can be type inferred.
  - Haskell, TypeScript, Rust, and Go have type inference in various degrees.
- structural type
  - nominal type has a few significant drawbacks:
    - create an inverted coupling between the producer and consumer.
      - this is the biggest problem of all.\
        Marking the application architecture rigit or very tedious to develop and maintain.
    - make the program unnecessary verbose and fill with needless type casts.
  - It is much easier to create a nominal type language vs a structural type language.
  -
- compile to native code
- explicit ownership, no garbage collection
- extensible through macro
- build-in linting and formatting
- workspace support
- filename conventions
- generics

Traits it gets from TypeScript:

- structural type
- type inference
- type composition
- syntax in TypeScript and JavaScript
  - generics

Traits unique to Uni:

- set theory type composition
- explicit syntax enables strong IDE support
- compiler engine is available during test
  - allow test to create isolated execution scope for dependency injection
