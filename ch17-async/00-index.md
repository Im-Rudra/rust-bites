# Chapter 17 — Async Rust: Notes Index

*Companion notes to The Rust Book, Ch. 17 (Fundamentals of Asynchronous Programming). Read in order.*

1. [Why Async Exists](01-why-async.md) — CPU-bound vs IO-bound; why threads waste waits
2. [What a Future Is](02-what-is-a-future.md) — lazy plans; `.await` executes the plan
3. [The Runtime](03-the-runtime.md) — the chicken-and-egg problem and the engine that cranks it
4. [Who Watches the Wait?](04-who-watches-the-wait.md) — interrupts, epoll, wakers: the doorbell chain *(deep-dive beyond the book)*
5. [Running Futures Concurrently](05-async-concurrency.md) — join, spawn_task, async channels
6. [Any Number of Futures](06-any-number-of-futures.md) — join!/join_all, race = cancellation, yield_now, DIY timeout
7. [Streams](07-streams.md) — async iterators; merge, throttle, timeout
8. [Under the Hood](08-under-the-hood.md) — the Future trait, poll, and why Pin exists
9. [Futures vs Tasks vs Threads](09-futures-tasks-threads.md) — the decision rule and the bridge pattern

**Naming note:** newer revisions of the online book renamed `trpl::run` → `trpl::block_on` and `trpl::race` → `trpl::select` (same behavior; old names still work as aliases).

**Chapter in one breath:** async fns return lazy plans (futures); a runtime cranks them; hardware doorbells wake the waiters; join/race/streams compose plans; Pin keeps self-pointing plans from moving; threads compute, async waits.
