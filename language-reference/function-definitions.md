# Function Definitions

Consideration:

- make all dependencies explicit
- access context properties as well as scope properties
  - `this` in class and parent scope
- instance of particular type have access to a fixed, controlled list of properties and methods
  - auto injecting function to object based on structural type does not work.
  - it leads to many conflicts and confusions
- provide a good organization for functions
  - namespace is a must in functional programming

```ebnf
fn = [decorators]
```

Pure function at top level:

```uni
// arrow function style
pub const add = (x: int, y: int) => x + y
pub const add = (x: int, y: int): int => x + y
// tuple `(x + y)` vs cast `(x + y) as int`
pub const add = (x: int, y: int) => (x + y) as int

// `fn` style
pub fn add(x: int, y: int) { x + y }
pub fn add(x: int, y: int): int { x + y }
pub fn add(x: int, y: int) { (x + y) as int }

// allow `fn` to omit `{}`
pub fn add(x: int, y: int) x + y
pub fn add(x: int, y: int): int x + y
pub fn add(x: int, y: int) (x + y) as int

// indentation
pub fn add(x: int, y: int)
  x + y
pub fn add(x: int, y: int): int
  x + y
pub fn add(x: int, y: int)
  (x + y) as int
```

Anomymous function in callback:

```uni
// arrow function style
[1, 2].map(x => x * 2)
[1, 2].map((x: int) => x * 2)
[1, 2].map((x): int => x * 2)
[1, 2].map((x: int): int => x * 2)
[1, 2].map(x => (x * 2) as int)

// `fn` style
[1, 2].map(fn (x) { x * 2 })
pub fn add(x: int, y: int) { x + y }
pub fn add(x: int, y: int): int { x + y }
pub fn add(x: int, y: int) { (x + y) as int }

// allow `fn` to omit `{}`
[1, 2].map(fn (x) x * 2)
[1, 2].map(fn (x: int) x * 2)
[1, 2].map(fn (x: int): int x * 2)
[1, 2].map(fn (x: int) (x * 2) as int)

// indentation
[1, 2].map(
  fn (x)
    x * 2
)
[1, 2].map(
  fn (x: int)
    x * 2
)
[1, 2].map(
  fn (x: int): int
    x * 2
)
[1, 2].map(
  fn (x: int)
    (x * 2) as int)
[
  fn (x)
    x * 2,
  fn (x)
    x + 2
]
```

With decorator:

```uni
// arrow function style
[1, 2].map(@log x => x * 2)

// `fn` style
[1, 2].map(@log fn (x) { x * 2 })

// allow `fn` to omit `{}`
[1, 2].map(@log fn (x) x * 2)

// indentation
[1, 2].map(
  @log
  fn (x)
    x * 2)
```

With context:

```uni
// arrow function style
pub const inc = (&ctx) => ctx.v += 1

// `fn` style
pub fn inc(&ctx) { ctx.v += 1 }

// allow `fn` to omit `{}`
pub fn inc(&ctx) ctx.v += 1

// indentation
pub fn inc(&ctx)
  ctx.v += 1

const counter = {
  inc: inc{this}
}
```

```uni
@log
add(x ; y) => x + y

@log
fn add(x ; y) {( x, y )}
fn add(x ; y) {[ x, y ]}
fn add(x ; y) {{ x, y }}
fn add(x ; y) {{ x }}
fn add(x ; y) {
  let x = 1
  {
    x // not evaluated as expression because single valuable is not an expression
  }
}
fn add(x ; y) {{ z: x + y }}
fn add(x ; y) {
  {
    z: x + y
  }
}

@log
let add = (x; y) => x + y
const add = (x; y) => {{z = y + x}}
const add = (x; y) => {
  {
    z = y + x
  }
}
const add = (x; y) => (x, y)
const add = (x; y) => [x, y]
fn add ctx = 1 x y z = ctx + x + y + z

add(s ; 1)

const obj = { x }
const add = {obj}() => { obj.}
add{s}()
obj.add()

const counter = {
  count: 0,
  inc() {
    this.count++
  }
}

const log = (target, &args, descriptor) => {
  args[0].borrow()
  log.info("calling ${descriptor.name}")
  target(...args)
}

struct Counter {
  count: int
}

mod Counter {
  pub fn new(): Counter {{ count: 0 }}
  pub fn new() {{count: 0} as Counter}
  @log
  pub fn inc(&mut ctx: Counter) { ctx.count++ }
  pub fn inc(&mut ctx) { ctx.count++ }
  pub let inc = (&mut ctx) => ctx.count++
}

struct Counter {
  let count: int

  pub fn new(): Counter {{ count: 0 }}
  pub fn new() {{count: 0} as Counter}

  @log
  pub fn inc(&mut ctx: Counter) { ctx.count++ }
  pub fn inc(&mut ctx) { ctx.count++ }
}

pub mod Counter
  pub count: int
  pub new(): Counter { count: 0 }
  @log
  pub inc(&mut ctx) ctx.count++
  pub use Other::*


struct Counter
  let count: int

  pub fn new(): Counter
    { count: 0 }
  pub fn new()
    {count: 0} as Counter

  @log
  pub fn inc(&mut ctx: Counter)
    ctx.count++
  pub fn inc(&mut ctx)
    ctx.count++

struct Counter
  let count: int

  pub fn new(): Counter
    { count: 0 }
  pub fn new()
    { count: 0 } as Counter

  @log
  pub fn inc(&mut ctx: Counter)
    ctx.count++
  pub fn inc(&mut ctx)
    ctx.count++


struct Counter {
  count: int
  new() { count: 0 }
  @log
  inc(&mut ctx) { ctx.count++ }
}

let c = { count: 0 }
Counter.inc(&c)

let x = Counter.new()
x.inc()


impl Counter {
  new: () => { count: 0 },
  inc
}

const inc = (&mut ctx: Counter) => ctx.count++



```

```rs
impl { v: i32 } {
  fn inc(&self, x: i32) {
    self.v += x
  }
}

struct Counter = {
  count: int
}

impl Counter {
  fn new() -> Counter {
    Counter{ count: 0 }
  }
  fn inc(&self) {
    self.count++
  }
}

fn inc(&self: Counter) {
  self.count++
}

inc(obj)
```

## Type Inference

Can type inference induce parametic polymorphism?

```uni
fn add(x, y) x + y

// identify types that can be used by `+`
fn<T: int | u8 | ...> add(x: T, y: T) x + y
```
