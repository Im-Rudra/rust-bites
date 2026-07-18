# 07 — Streams (async iterators)

*Rust Book, Ch. 17.4. Builds on: 06 — Any Number of Futures.*

## The idea in one line

An **Iterator** is a box of items already sitting there. A **Stream** is a conveyor belt — items *arrive over time*, and you wait at the end of the belt:

| | get next item |
|---|---|
| Iterator | `next()` — instant |
| Stream | `next().await` — may wait |

```rust
use trpl::StreamExt;

let mut stream = trpl::stream_from_iter(vec![1, 2, 3].into_iter());
while let Some(v) = stream.next().await {
    println!("{v}");
}
```

## Where streams come from

- `trpl::stream_from_iter(iter)` — upgrade any iterator.
- `trpl::ReceiverStream::new(rx)` — wrap a channel receiver: now incoming messages are a stream.
- Real life: incoming network requests, lines from a file, GUI events — all naturally streams.

## The toolbox (StreamExt)

Iterator's adapters, async-flavored: `map`, `filter`, plus time-aware ones —

- `.merge(other)` — two belts feeding one.
- `.throttle(duration)` — slow the belt down (pulls at most one item per interval).
- `.timeout(duration)` — each `next().await` now yields `Result`: `Err` if the item took too long. **The stream doesn't die** — with a buffering (unbounded) channel underneath, the late item shows up on the next pull.

## Fine print

- `next()` lives on the `StreamExt` trait — the `use trpl::StreamExt;` import is easy to forget (the compiler will tell you).
- After `.timeout()`, the resulting stream must be **pinned** before polling — `let mut s = std::pin::pin!(stream.timeout(d));` (why pinning: note 08).
- Pattern to remember: `while let Some(x) = stream.next().await` is the async `for` loop.

**One-liner:** A stream is an iterator whose `next` may need waiting — same adapters, plus time-aware ones.

🔨 **Lab:** [labs/lab-07-streams](labs/lab-07-streams/)
