# Chapter 17 Labs

Real cargo projects, not snippets. Each lab: starter code that compiles, with `TODO` holes you fill. 10–20 minutes each.

## How to run

```
cd lab-01-03-lazy-proof
cargo run
```

## The rules

1. Read the matching note first (the lab name tells you which: `lab-05-...` ↔ note `05-...`).
2. Fill the TODOs in order. When the compiler yells — **read the error slowly**. The error *is* the lesson.
3. Stuck 15+ minutes? Peek at `_solutions/<lab-name>.rs`. Not before.

## Map

| Lab | Notes | Proves |
|---|---|---|
| lab-01-03-lazy-proof | 01, 02, 03 | futures run nothing until awaited; only a runtime can start the dance |
| lab-05-join-channels | 05 | join interleaves (A1 B1 A2 B2...); channels + the `async move` trap |
| lab-06-race-timeout | 06 | select races futures; losers get cancelled; build your own timeout |
| lab-07-streams | 07 | iterator → stream, channel → stream, `while let ... next().await` |
| lab-09-bridge | 09 | a thread computes while async stays responsive; channel marries them |

Notes 04 (doorbell) and 08 (Pin) are theory — no labs; you'll feel them working inside every lab above.
