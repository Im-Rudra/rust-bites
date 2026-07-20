# 08 — Lifetime Elision and 'static

*Rust Book, 10.3. Builds on: 07 — Lifetime Annotations.*

## Why you rarely write 'a

This compiles without annotations — but note 07 said references in and out need them?

```rust
fn first_word(s: &str) -> &str { ... }   // ✅ fine. why?
```

In pre-1.0 Rust everyone wrote `fn first_word<'a>(s: &'a str) -> &'a str` — *every time*. The patterns were so predictable that the team baked them into the compiler as the **elision rules**. If the rules resolve every lifetime, you write nothing; if ambiguity remains (like `longest`), the compiler asks you.

## The three rules

1. **Each input reference gets its own lifetime.** `fn f(x: &str, y: &str)` → `<'a, 'b>`.
2. **Exactly one input lifetime? It's assigned to every output.** That's why `first_word` works: one input, so the output obviously borrows from it.
3. **`&self` or `&mut self` in the mix? self's lifetime goes to all outputs.** That's why methods almost never need annotations — returning something borrowed from `self` is the overwhelmingly common case.

`longest` fails rule 2 (two inputs) and rule 3 (no self) → you annotate. Now the error message makes sense instead of feeling random.

## `'static` — the whole-program lifetime

```rust
let s: &'static str = "I have a static lifetime.";
```

String literals live in the binary itself — valid for the entire run, hence `'static`.

**The trap:** compiler errors sometimes *suggest* `'static`. Before obeying, ask: does this data truly live forever, or did I create a dangling reference and the compiler is offering a (wrong) way to silence it? Usually the real fix is restructuring what owns what — not immortality.

**One-liner:** Elision auto-fills the obvious lifetimes (own, one-in→out, self→out); `'static` means "lives forever" — suspect any error that suggests it.

🔨 **Lab:** [labs/lab-06-09-lifetimes](labs/lab-06-09-lifetimes/) *(covers notes 06–09)*
