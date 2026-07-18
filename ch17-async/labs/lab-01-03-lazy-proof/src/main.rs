// LAB 01–03 — Lazy Proof
// Matching notes: 01-why-async, 02-what-is-a-future, 03-the-runtime
//
// Goal — prove with your own eyes that:
//   (1) calling an async fn runs NOTHING,
//   (2) .await is what runs it,
//   (3) only a runtime can drive the outermost future.
//
// Fill the TODOs in order, `cargo run` after each.

use std::time::Duration;

// TODO(1): make this fn print "loud_number is RUNNING!" and return 42.
async fn loud_number() -> u32 {
    todo!("print something loud, then return 42")
}

fn main() {
    // TODO(2): call loud_number() and store the result in a variable `fut`.
    // Do NOT await it. (Notice the compiler warning you'll get — read it!)

    println!("future created. watch: NOTHING printed above this line.");
    std::thread::sleep(Duration::from_secs(1));
    println!("one second passed. still silent. futures are LAZY.");

    // TODO(3): start the engine — trpl::block_on(async { ... }) — and inside:
    //     let n = fut.await;
    //     println!("got {n}");
    // Only now should "loud_number is RUNNING!" appear.

    // BONUS: before fixing it, try writing `fut.await` directly here in main,
    // OUTSIDE block_on. Read error E0728 carefully — it's note 03 in compiler form.
}
