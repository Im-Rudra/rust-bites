# 05 — Running Futures Concurrently (join, spawn_task, channels)

*Rust Book, Ch. 17.2. Builds on: 03 — The Runtime.*

## Two ways to do "at the same time"

1. **`trpl::join(a, b)`** — one task drives two futures, weaving between them. One waiter serving two tables: takes an order here, while the kitchen cooks, takes an order there.
2. **`trpl::spawn_task(...)`** — a brand-new independent task. Hiring a second waiter. (Async's version of `thread::spawn` — but tasks cost bytes, not megabytes.)

```rust
trpl::run(async {
    let fut1 = async {
        for i in 1..5 { println!("A{i}"); trpl::sleep(Duration::from_millis(500)).await; }
    };
    let fut2 = async {
        for i in 1..5 { println!("B{i}"); trpl::sleep(Duration::from_millis(500)).await; }
    };
    trpl::join(fut1, fut2).await;   // output interleaves: A1 B1 A2 B2 ...
});
```

## Key facts

- `join` waits for **both** to finish; switching happens **at await points** — this interleaving can run on a single thread.
- A future in a variable still does nothing (lazy!) — `join` is what drives both.
- `spawn_task` returns a handle (itself a future). If the main future ends first, unfinished spawned tasks are **dropped mid-flight** — `.await` the handle to keep them alive.

## Talking between tasks: channels

Async version of Ch. 16's mpsc — `recv` awaits instead of blocking:

```rust
let (tx, mut rx) = trpl::channel();
tx.send("hi").unwrap();                       // send: not async
while let Some(msg) = rx.recv().await { ... } // recv: awaits
```

## Fine print

- The loop ends when the channel closes, and the channel closes **when every `tx` is dropped**. Classic bug: `tx` moved into an `async` block *without* `move` stays owned by the outer scope → never dropped → `rx.recv().await` waits forever. Fix: `async move`.
- Clone `tx` for multiple producers, same as Ch. 16.

**One-liner:** `join` weaves futures inside one task; `spawn_task` adds another worker; channels let them talk.

🔨 **Lab:** [labs/lab-05-join-channels](labs/lab-05-join-channels/)
