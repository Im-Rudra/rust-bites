# 09 — All Together: Generics + Traits + Lifetimes in One Signature

*Rust Book, 10.3 finale. The chapter's three ideas in a single function.*

## The boss signature

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
```

## Read it piece by piece (this is the skill)

| Piece | From note | Meaning |
|---|---|---|
| `<'a, T>` | 02, 07 | two placeholders: a lifetime and a type — lifetimes ARE generics, they share the `<>` |
| `x: &'a str, y: &'a str` | 07 | result's leash tied to the shorter-lived input |
| `ann: T` + `where T: Display` | 05 | any printable type may be announced |
| `-> &'a str` | 07 | returns a borrow, valid while both inputs live |

Nothing new — just all three vocabularies spoken in one sentence. If you can read this signature aloud in plain English, Chapter 10 is yours.

## Where you go from here

- These three tools power *everything* ahead: `Box<dyn Trait>` (Ch. 18), iterator adapters (Ch. 13), async's `Future` trait (Ch. 17) — all traits + generics underneath.
- Lifetimes go quiet again after this chapter — elision handles the bulk; you'll annotate mostly in structs holding references and multi-input returning functions.

**One-liner:** `<'a, T: Display>` — a lifetime, a type, a contract: the whole chapter in one pair of angle brackets.

🔨 **Lab:** [labs/lab-06-09-lifetimes](labs/lab-06-09-lifetimes/) *(covers notes 06–09)*
