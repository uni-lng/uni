# rust enum

it's the same as nominal union type.

```rs
enum Option<T> {
  Some<T>,
  None
}
```

```just
nominal type Some<T> = Symbol<T>()
nominal type None = Symbol()

nominal type Option<T> = Some<T> | None
```

Sub question, what is `Some<T>`?
