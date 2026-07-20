# 05 — Trait Bounds: Generics That Can DO Things

*Rust Book, 10.2, second half. Builds on: 03 + 04.*

## The marriage of generics and traits

Note 02's `largest<T>` didn't compile — a bare `T` can be *any* type, and not every type can be compared with `>`. A **trait bound** narrows "any type" to "any type that signed this contract":

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {   // T must be comparable
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }   // now legal: PartialOrd guarantees >
    }
    largest
}
```

Bound = permission slip. No bound, no method calls on `T`.

## The three spellings (equivalent for one param — not when params must share a type)

```rust
pub fn notify(item: &impl Summary)                 // sugar — quick & common
pub fn notify<T: Summary>(item: &T)                // full form — needed when two params must be the SAME T
pub fn notify<T>(item: &T) where T: Summary        // where-clause — keeps complex signatures readable
```

Multiple contracts: `T: Summary + Display`.

## Returning "some type that implements..."

```rust
fn make_summarizable() -> impl Summary { SocialPost { /* ... */ } }
```

The caller knows only "it can summarize" — the concrete type stays hidden. Limit: the body must return **one single concrete type** (an `if/else` returning `NewsArticle` or `SocialPost` won't compile — that needs Ch. 18's trait objects).

## Conditional powers

```rust
struct Pair<T> { x: T, y: T }

impl<T: Display + PartialOrd> Pair<T> {   // ONLY comparable+printable Pairs...
    fn cmp_display(&self) { ... }         // ...get this method
}
```

And the ecosystem-wide version — **blanket impls**: `impl<T: Display> ToString for T` is how *everything* printable gets `.to_string()` for free — that's why `3.to_string()` just works.

**One-liner:** A bound turns "any type" into "any type that can X" — that's what lets generic code actually call methods.

🔨 **Lab:** [labs/lab-04-05-traits](labs/lab-04-05-traits/) *(covers notes 04–05)*
