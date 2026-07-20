# Chapter 10 — Generic Types, Traits, and Lifetimes: Notes Index

*Companion notes to The Rust Book, Ch. 10. Read in order.*

1. [Why Generics Exist](01-why-generics.md) — the de-duplication ladder: values → variables, code → functions, types → generics
2. [Generic Data Types](02-generic-data-types.md) — `<T>` in functions, structs, enums, methods
3. [Monomorphization](03-monomorphization.md) — the cookie cutter: why generics cost nothing at runtime
4. [Traits](04-traits.md) — contracts of behavior; default implementations; the orphan rule
5. [Trait Bounds](05-trait-bounds.md) — generics that can DO things; impl Trait; blanket impls
6. [Why Lifetimes Exist](06-why-lifetimes.md) — dangling references and the borrow checker's one law
7. [Lifetime Annotations](07-lifetime-annotations.md) — the `longest` function; what `'a` really says
8. [Elision and 'static](08-elision-and-static.md) — the three auto-fill rules; the 'static trap
9. [All Together](09-all-together.md) — one signature speaking all three vocabularies

**Labs:** hands-on cargo projects in [labs/](labs/) — `lab-01-03-generic-largest`, `lab-04-05-traits`, `lab-06-09-lifetimes` (name = the notes it matches). Start with [labs/README.md](labs/README.md).

**Chapter in one breath:** generics abstract types (and vanish at compile time via monomorphization); traits are contracts that bound what a `T` can do; lifetimes are generics over *how long references live*, annotated only when the compiler can't infer the leash.
