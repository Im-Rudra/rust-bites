// SOLUTION — lab-07-streams
use std::time::Duration;
use trpl::{ReceiverStream, StreamExt};

fn main() {
    trpl::block_on(async {
        // PART 1: iterator → stream
        let mut doubled = trpl::stream_from_iter((1..=10).map(|n| n * 2));
        while let Some(v) = doubled.next().await {
            println!("doubled: {v}");
        }

        // PART 2: channel → stream
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            for msg in ["hello", "from", "a", "stream"] {
                tx.send(msg).unwrap();
                trpl::sleep(Duration::from_millis(300)).await;
            }
        }); // tx dropped when the task ends → channel closes → stream ends

        let mut messages = ReceiverStream::new(rx);
        while let Some(m) = messages.next().await {
            println!("message: {m}");
        }
        // The loop exits because the channel closed (tx dropped) —
        // a closed channel = the stream returns None = the belt stopped.
    });
}
