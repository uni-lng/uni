# Attach function to type

Language service will attach a function of type `(x: &T) => any` and `(x: &mut T) => any`,
to an instance of type `T` if the `function symbol` is in scope.

This means given the example below:

```just
// cursor.just
type Cursor = { a: string }

let advanceToken = (this: &mut Cursor) => {  /* -snip- */ }
```

The `advanceToken` function can be attached on any instance of `Cursor`.

```just
// cursor.just
let cur: Cursor = { a: "a" }

cur.advanceToken()
```

The `function symbol` can be loaded in three scenarios:

1. The function is part of the standard core which is loaded automatically.
2. The function id declared in the same file.
3. The function is included by the `use` statement

Note that it is available in code completion regardless,
so that IDE can complete it and add the `use` statement automatically.

```just
adv
// ^ code complete will show `advanceToken`
```
