# Chapter 10 Labs

Real cargo projects. Starter code compiles (except where a TODO tells you to *watch it fail* — those errors are the lesson). No external dependencies — pure std.

## How to run

```
cd lab-01-03-generic-largest
cargo run
```

## The rules

1. Read the matching notes first (`lab-04-05-...` ↔ notes 04–05).
2. Fill TODOs in order. Read every compiler error slowly — Ch. 10 is where Rust's errors become your teacher.
3. Stuck 15+ minutes? `_solutions/<lab-name>.rs`. Not before.

## Map

| Lab | Notes | Proves |
|---|---|---|
| lab-01-03-generic-largest | 01–03 | one generic fn replaces per-type copies; bounds preview; specialization via `impl Point<f32>` |
| lab-04-05-traits | 04–05 | contracts, default methods, the three bound spellings, conditional methods |
| lab-06-09-lifetimes | 06–09 | dangling refs, fixing `longest`, structs with refs, the boss signature |
