# 06 — Any Number of Futures (join!, join_all, race, yield)

*Rust Book, Ch. 17.3. Builds on: 05 — Running Futures Concurrently.*

## Scaling up `join`

- **`trpl::join!(a, b, c)`** — macro; any *fixed* number of futures, types may differ.
- **`trpl::join_all(vec)`** — a *dynamic* number, but all must be the **same type**. Two `async` blocks are never the same type, so you erase them into trait objects:

```rust
let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
    vec![Box::pin(a), Box::pin(b), Box::pin(c)];
trpl::join_all(futures).await;
```

(Why `Pin<Box<...>>` and not just `Box<...>`? → note 08.)

## race — first one wins

```rust
trpl::race(fast, slow).await   // returns Either::Left / Either::Right
```

- The moment one future finishes, the other is **dropped** — and dropping a future **cancels** it. That's how async cancellation works: no kill signal, just drop the plan.
- Fine print: `trpl::race` isn't fair — it polls arguments in order. (Newer revisions of the book call it `trpl::select`.)

## The dark side of cooperative scheduling

Switching happens **only at await points**. A future that crunches numbers for 5 seconds without a single `.await` holds its thread hostage — every other future sharing that thread starves. The polite fix inside long loops:

```rust
trpl::yield_now().await;   // "I'm not done, but someone else can go"
```

## Build your own timeout (race + sleep)

```rust
async fn timeout<F: Future>(f: F, max: Duration) -> Result<F::Output, Duration> {
    match trpl::race(f, trpl::sleep(max)).await {
        Either::Left(out) => Ok(out),
        Either::Right(_)  => Err(max),
    }
}
```

Small pieces (`race`, `sleep`) compose into real tools — this is the async ecosystem's whole design style.

**One-liner:** `join` = everyone finishes; `race` = first wins, losers are dropped (= cancelled); switching only happens at `.await`.

🔨 **Lab:** [labs/lab-06-race-timeout](labs/lab-06-race-timeout/)
