// SOLUTION — lab-09-bridge
use std::time::Duration;

fn heavy_compute(n: u64) -> u64 {
    (0..n).sum()
}

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // real OS thread does the CPU work
        std::thread::spawn(move || {
            let result = heavy_compute(1_000_000_000);
            tx.send(result).unwrap();
        }); // tx dropped after sending → channel closes

        let heartbeat = async {
            loop {
                println!("async side responsive...");
                trpl::sleep(Duration::from_millis(200)).await;
            }
        };

        let waiter = async {
            while let Some(result) = rx.recv().await {
                println!("result = {result}");
            }
        };

        // waiter ends when the channel closes; select then DROPS the endless
        // heartbeat — using the race as an off-switch.
        match trpl::select(waiter, heartbeat).await {
            _ => println!("done."),
        }

        // If you call heavy_compute() directly in async code instead:
        // the heartbeat freezes — blocking code holds the worker thread hostage,
        // and every future sharing it starves. Threads work; async waits.
    });
}
