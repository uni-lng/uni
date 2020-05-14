# Curry

Curry is a way to create a new function by filling in part of the parameters of an existing function.

The syntax is `<fn>{<curry params>}`.

```uni
const add = (x: int, y: int) => x + y
const add5 = add(5) // add5: (y: int) -> int

add5(3) // 8
```

The curry syntax is also used in binding functions to object and struct:

```uni
const decToZero = fn (&ctx: { value: int }, v: int) { ctx.value = Math.max(ctx.value - v, 0) }

const state = {
  value: 5,
  inc: inc {
    Math: {
      max: myMax
    }
  }(&self)
}

struct State {
  value: int
  inc: inc(&self)
}
```
