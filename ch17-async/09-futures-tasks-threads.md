# 09 — Futures vs Tasks vs Threads: Which One, When

*Rust Book, Ch. 17.6. Closes the chapter — builds on everything before.*

## The hierarchy

- **Thread** — owned by the OS. Preemptive (the OS can interrupt it anywhere, no cooperation needed). Heavy: ~2 MB of stack each (Rust's default).
- **Task** — owned by the runtime. Cooperative (yields only at `.await`). Cheap: just bytes — spawn millions.
- **Future** — the unit of work inside a task. A task drives a tree of futures.

The runtime itself runs tasks *on* a small pool of threads (Tokio even steals tasks between threads to balance load) — so these layers stack, they don't compete.

## The decision rule

| Work is... | Use |
|---|---|
| computing (CPU-bound) | threads |
| waiting (IO-bound) | async |
| massive # of connections | async (threads can't scale there) |
| both at once | combine them |

## Combining: the bridge pattern

Heavy computation on a thread; the async world awaits the result through a channel — no thread blocked, no task starved:

```rust
let (tx, mut rx) = trpl::channel();

std::thread::spawn(move || {
    let result = heavy_compute();      // hog a CPU core, guilt-free
    tx.send(result).unwrap();
});

while let Some(result) = rx.recv().await {   // async side waits without blocking
    println!("{result}");
}
```

## Fine print

- Never call blocking functions (`std::thread::sleep`, sync file IO, heavy loops) inside async code — you freeze the worker thread and every task on it. Move the blocking part to a thread, bridge with a channel.
- Ecosystem versions of this pattern: `tokio::task::spawn_blocking`, `tokio::sync::mpsc`.

**One-liner:** Threads for working, async for waiting — and channels marry the two.

🔨 **Lab:** [labs/lab-09-bridge](labs/lab-09-bridge/)
