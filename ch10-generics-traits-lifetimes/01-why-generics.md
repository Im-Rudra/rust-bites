# 01 — Why Generics Exist

*Rust Book, Ch. 10 intro. Prerequisite: functions, structs, enums (Ch. 3–6).*

## The ladder of de-duplication

You've already climbed two rungs of this ladder without noticing:

1. Same **value** used twice → extract a **variable**.
2. Same **code** run twice → extract a **function**.
3. Same function written for two **types** → extract a **generic**. ← this chapter

## The book's running example

Find the largest number in a list. Then in a *second* list — copy-paste. Extract a function:

```rust
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
```

Nice. Now find the largest `char`... and the only difference from the `i32` version would be the *type in the signature*. The bodies are identical. Copy-pasting per type is rung-2 thinking; generics are rung 3:

```rust
fn largest<T>(list: &[T]) -> &T   // T = "some type, to be named later"
```

## What this chapter builds

- **Generics** — code with type placeholders (`T`).
- **Traits** — constraints on what a `T` must be able to *do* (spoiler: the function above needs `T` to be comparable).
- **Lifetimes** — a special kind of generic describing how long *references* live.

**One-liner:** Variables abstract values, functions abstract code, generics abstract types.

🔨 **Lab:** [labs/lab-01-03-generic-largest](labs/lab-01-03-generic-largest/) *(covers notes 01–03)*
