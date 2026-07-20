# 02 — Generic Data Types

*Rust Book, 10.1. Builds on: 01 — Why Generics Exist.*

## The one syntax, four places

Declare the placeholder in `<>` right after the name — then use it like a type:

```rust
// 1. Functions
fn largest<T>(list: &[T]) -> &T { ... }

// 2. Structs
struct Point<T> { x: T, y: T }              // both fields SAME type T
struct Point2<T, U> { x: T, y: U }          // fields may differ

// 3. Enums — you've used these since Ch. 6!
enum Option<T>       { Some(T), None }
enum Result<T, E>    { Ok(T), Err(E) }

// 4. Methods — note the <T> after impl
impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
}
```

## Reading `impl<T>`

`impl<T> Point<T>` = "for *every* type T, Point of that T gets these methods."
Compare: `impl Point<f32>` (no `<T>` after impl) = "only `Point<f32>` gets these" — you can give special abilities to specific concrete versions.

## The trap that teaches

```rust
let wont_work = Point { x: 5, y: 4.0 };  // ❌ i32 and f64 into the same T
```

`Point<T>` promised x and y are the *same* type. The compiler infers `T = i32` from `x`, then rejects `y`. Want mixed? That's what `Point2<T, U>` is for.

## Fine print

- Method type params can differ from struct type params — a method can introduce its *own* `<X>` for values it takes.
- Convention: `T` (type), `E` (error), `K`/`V` (key/value) — any identifier works; UpperCamelCase is the convention.

**One-liner:** Declare the placeholder in `<>`, then structs, enums, functions, and methods all speak "some type T."

🔨 **Lab:** [labs/lab-01-03-generic-largest](labs/lab-01-03-generic-largest/) *(covers notes 01–03)*
