# 04 — Traits: Defining Shared Behavior

*Rust Book, 10.2, first half. Builds on: 02 — Generic Data Types.*

## The idea

A **trait** is a contract: "any type signing this contract can do these things."

```rust
pub trait Summary {
    fn summarize(&self) -> String;   // signature only — each signer writes its own body
}
```

Types sign the contract with `impl ... for`:

```rust
pub struct NewsArticle { pub headline: String, pub author: String, /* ... */ }

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct SocialPost { pub username: String, pub content: String, /* ... */ }

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
```

Same contract, different behavior per type — that's the point.

## Default implementations

A trait method can carry a ready-made body; signers get it for free and may override:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;          // required
    fn summarize(&self) -> String {                // default — can call required methods!
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String { format!("@{}", self.username) }
    // summarize comes free
}
```

Powerful pattern: a trait with one small required method can offer many derived methods for free.

## Fine print — the orphan rule

You can `impl Trait for Type` only if **the trait or the type is local to your crate**. You can't impl `Display` on `Vec<T>` (both foreign) — otherwise two crates could write conflicting impls and Rust couldn't pick. This is the *orphan rule*, part of a property called *coherence*.

**One-liner:** A trait is a signed contract of behavior — each type provides its own fulfillment, defaults come free.

🔨 **Lab:** [labs/lab-04-05-traits](labs/lab-04-05-traits/) *(covers notes 04–05)*
