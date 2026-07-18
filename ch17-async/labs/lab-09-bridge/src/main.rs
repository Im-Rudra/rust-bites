// LAB 09 — The Bridge: threads for working, async for waiting
// Matching note: 09-futures-tasks-threads
//
// Goal: a thread grinds a heavy computation while the async side
// stays alive and responsive — married through a channel.

use std::time::Duration;

fn heavy_compute(n: u64) -> u64 {
    (0..n).sum() // pretend this is video encoding
}

fn main() {
    trpl::block_on(async {
        // (the ::<u64> annotation is only needed while the TODOs are empty —
        //  once you actually send a u64, the compiler can infer it and you may drop it)
        let (tx, mut rx) = trpl::channel::<u64>();

        // TODO(1): spawn a REAL OS thread (std::thread::spawn + move closure) that:
        //   - computes heavy_compute(1_000_000_000)
        //   - sends the result: tx.send(result).unwrap()

        // TODO(2): a "heartbeat" future that proves async is still alive:
        //   loop { println!("async side responsive..."); trpl::sleep(200ms).await; }

        // TODO(3): a "waiter" future that receives the result:
        //   while let Some(result) = rx.recv().await { println!("result = {result}"); }

        // TODO(4): trpl::select(waiter, heartbeat).await
        //   — when the waiter finishes, the endless heartbeat is dropped. Race as an off-switch!

        // THINK: replace TODO(1)'s thread with plain heavy_compute() called directly
        // inside this async block. What happens to the heartbeat, and why?
        // (Note 09's fine print: blocking in async freezes the whole worker.)
    });
}
