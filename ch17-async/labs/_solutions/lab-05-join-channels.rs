// SOLUTION — lab-05-join-channels
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // PART 1
        let fut_a = async {
            for i in 1..=4 {
                println!("A{i}");
                trpl::sleep(Duration::from_millis(300)).await;
            }
        };
        let fut_b = async {
            for i in 1..=4 {
                println!("B{i}");
                trpl::sleep(Duration::from_millis(300)).await;
            }
        };
        trpl::join(fut_a, fut_b).await; // A1 B1 A2 B2 ...

        // PART 2
        let (tx, mut rx) = trpl::channel();

        let sender = async move { // `move` takes tx INSIDE, so it drops when done
            for msg in ["one", "two", "three"] {
                tx.send(msg).unwrap();
                trpl::sleep(Duration::from_millis(300)).await;
            }
        }; // tx dropped here → channel closes → receiver loop can end

        let receiver = async {
            while let Some(m) = rx.recv().await {
                println!("got {m}");
            }
        };

        trpl::join(sender, receiver).await;

        // Without `move`: tx is only BORROWED by the sender block, so it lives on
        // in the outer scope, never drops, the channel never closes, and
        // rx.recv().await waits forever → the program hangs.
    });
}
