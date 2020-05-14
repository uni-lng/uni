# Injection

Function receive input in two ways:

- parameters
- scope access

Take the following example:

```uni
const x = 1

fn inc(a: int) {
  return x + a
}
```

In the above example, the function `inc()` has two inputs: `a` and `x`.

`a` is a paramater to `inc()`, which is very straight forward.
`x` is available to `inc()` due to scope rules.

The references in the scope accessed by the function is captured with the function itself,
by default these references will be used and you can call the function as normal:

```uni
inc(38) // 39
```

However, you can also replace the references with the `inject` syntax:

```uni
inc{ x: 10 }(2) // 12
```

This mechanism elminates the need of dependency injection boilerplates needed in other languages. And make testing any function extremely straight forward.
