// SOLUTION — lab-01-03-lazy-proof
use std::time::Duration;

async fn loud_number() -> u32 {
    println!("loud_number is RUNNING!");
    42
}

fn main() {
    let fut = loud_number(); // nothing runs — it's a plan

    println!("future created. watch: NOTHING printed above this line.");
    std::thread::sleep(Duration::from_secs(1));
    println!("one second passed. still silent. futures are LAZY.");

    trpl::block_on(async {
        let n = fut.await; // NOW it runs
        println!("got {n}");
    });
}
