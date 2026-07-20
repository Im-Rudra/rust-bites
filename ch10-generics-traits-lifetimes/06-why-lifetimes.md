# 06 — Why Lifetimes Exist

*Rust Book, 10.3, opening. Builds on: references & borrowing (Ch. 4).*

## The crime lifetimes prevent

A **dangling reference** — pointing at memory whose owner is gone:

```rust
fn main() {
    let r;                    // declared in the outer scope
    {
        let x = 5;
        r = &x;               // r borrows x...
    }                         // ...x DIES here
    println!("r: {r}");       // ❌ r points at a corpse
}
```

```
error[E0597]: `x` does not live long enough
```

## How the compiler catches it

Every reference already has a **lifetime** — the region of code it's valid for. The **borrow checker** compares them:

- `x` lives for the inner scope only (call it `'b`).
- `r` is *used* over the outer scope (call it `'a`) — longer than `'b`.
- A reference to something with lifetime `'b`, used across `'a` where `'a` outlives `'b`? **Rejected.**

The subject of the reference must live **at least as long** as the reference is used. That's the whole law.

## The key reframe

You've obeyed lifetimes since Chapter 4 — the borrow checker just inferred everything silently. This section of the book exists because *sometimes the compiler can't infer* which input a returned reference came from... and then it asks you to annotate. That's all lifetime syntax is: **answering the compiler's question, not changing any behavior**.

**One-liner:** Every reference has a lifetime; the borrow checker's one law is "the data must outlive the reference."

🔨 **Lab:** [labs/lab-06-09-lifetimes](labs/lab-06-09-lifetimes/) *(covers notes 06–09)*
