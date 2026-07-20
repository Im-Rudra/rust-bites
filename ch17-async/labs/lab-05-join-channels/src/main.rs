// LAB 05 — Join & Channels
// Matching note: 05-async-concurrency
//
// Part 1 goal: see interleaving (A1 B1 A2 B2 ...) with your own eyes.
// Part 2 goal: message passing — and the famous `async move` trap.

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ---------- PART 1: join ----------
        let fut_a = async {
            for i in 1..=4 {
                println!("A{i}");
                trpl::sleep(Duration::from_millis(300)).await;
            }
        };

        // TODO(1): write fut_b — same shape, prints B1..B4.

        // TODO(2): drive BOTH with trpl::join(fut_a, fut_b).await
        // Expected output: A1 B1 A2 B2 A3 B3 A4 B4 — switching at every await.

        // ---------- PART 2: channels ----------
        // TODO(3): create a channel: let (tx, mut rx) = trpl::channel();

        // TODO(4): a sender future — `async move` block that:
        //   for msg in ["one", "two", "three"] { tx.send(msg).unwrap(); sleep 300ms }

        // TODO(5): a receiver future:
        //   while let Some(m) = rx.recv().await { println!("got {m}"); }

        // TODO(6): trpl::join(sender, receiver).await

        // QUESTION (the trap): delete `move` from the sender block and run again.
        // The program never exits. Explain why in one sentence, out loud.
        // (Answer is in note 05's fine print.)
    });
}
