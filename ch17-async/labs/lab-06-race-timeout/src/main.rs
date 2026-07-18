// LAB 06 — Race & Build-Your-Own Timeout
// Matching note: 06-any-number-of-futures
//
// Goal: race two futures, watch the loser get CANCELLED (dropped),
// and compose select + sleep into a real timeout utility.

use std::{future::Future, time::Duration};
use trpl::Either;

async fn slow_job() -> &'static str {
    trpl::sleep(Duration::from_secs(2)).await;
    "finally done"
}

// TODO(1): implement timeout by RACING `f` against trpl::sleep(max):
//   trpl::select(f, trpl::sleep(max)).await returns an Either —
//   Left(output)  => the job won  => Ok(output)
//   Right(())     => the clock won => Err(max)
async fn timeout<F: Future>(f: F, max: Duration) -> Result<F::Output, Duration> {
    todo!()
}

fn main() {
    trpl::block_on(async {
        // TODO(2): call timeout(slow_job(), 500 ms) and print the result.
        // Expected: Err — the job needed 2s, the clock won, the job was dropped.

        // TODO(3): call timeout(slow_job(), 5 s) and print the result.
        // Expected: Ok("finally done").

        // THINK: in TODO(2), where did the half-finished slow_job go?
        // Nobody "stopped" it — it was dropped. Drop = cancellation. (Note 06.)
    });
}
