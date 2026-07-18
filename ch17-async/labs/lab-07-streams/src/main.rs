// LAB 07 — Streams
// Matching note: 07-streams
//
// Part 1 goal: upgrade an iterator to a stream and consume it the async way.
// Part 2 goal: turn a channel into a stream — values arriving over time.

use std::time::Duration;
use trpl::{ReceiverStream, StreamExt};

fn main() {
    trpl::block_on(async {
        // ---------- PART 1: iterator → stream ----------
        // TODO(1): let mut doubled = trpl::stream_from_iter((1..=10).map(|n| n * 2));

        // TODO(2): consume it:
        //   while let Some(v) = doubled.next().await { println!("doubled: {v}"); }
        // (This is the async `for` loop — memorize the shape.)

        // ---------- PART 2: channel → stream ----------
        // TODO(3): let (tx, rx) = trpl::channel();

        // TODO(4): spawn a task that feeds the channel, then hangs up:
        //   trpl::spawn_task(async move {
        //       for msg in ["hello", "from", "a", "stream"] {
        //           tx.send(msg).unwrap();
        //           trpl::sleep(Duration::from_millis(300)).await;
        //       }
        //   });   // tx dropped here => channel closes => stream will END

        // TODO(5): wrap and consume:
        //   let mut messages = ReceiverStream::new(rx);
        //   while let Some(m) = messages.next().await { println!("message: {m}"); }

        // THINK: why does the Part 2 loop exit at all?
        // (What closed the channel? Note 05's fine print, meeting note 07.)
    });
}
