# 03 — Monomorphization (why generics cost nothing)

*Rust Book, 10.1, final section. Builds on: 02 — Generic Data Types.*

## The fear

"If `largest<T>` works for any type... doesn't the program have to figure out the type *while running*? Isn't that slow?"

## The answer: the compiler is a cookie cutter

At compile time, Rust finds every concrete type you actually used, and **stamps out a dedicated copy** of the generic code for each. This is *monomorphization* — "making many forms into single forms."

You write:

```rust
let integer = Some(5);
let float = Some(5.0);
```

The compiler ships (conceptually):

```rust
enum Option_i32 { Some(i32), None }
enum Option_f64 { Some(f64), None }
```

Two separate, fully concrete types. By runtime, **generics no longer exist** — only the stamped copies do.

## Why this matters to you

- **Zero runtime cost.** The generic version runs exactly as fast as if you had hand-copy-pasted a version per type. You get rung-3 ergonomics at rung-2 speed.
- This is a recurring Rust theme you'll keep meeting: *zero-cost abstractions* — pay at compile time, run at full speed.

## Fine print

- The cost that *does* exist: compile time and binary size grow with each stamped copy. (Ch. 18 shows the alternative — trait objects — which trade a tiny runtime cost for one shared copy.)

**One-liner:** The compiler stamps a concrete copy per used type — generics vanish before runtime, costing nothing.

🔨 **Lab:** [labs/lab-01-03-generic-largest](labs/lab-01-03-generic-largest/) *(covers notes 01–03)*
