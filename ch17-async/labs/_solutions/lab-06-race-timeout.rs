// SOLUTION — lab-06-race-timeout
use std::{future::Future, time::Duration};
use trpl::Either;

async fn slow_job() -> &'static str {
    trpl::sleep(Duration::from_secs(2)).await;
    "finally done"
}

async fn timeout<F: Future>(f: F, max: Duration) -> Result<F::Output, Duration> {
    match trpl::select(f, trpl::sleep(max)).await {
        Either::Left(output) => Ok(output), // the job won
        Either::Right(_) => Err(max),       // the clock won; job dropped = cancelled
    }
}

fn main() {
    trpl::block_on(async {
        match timeout(slow_job(), Duration::from_millis(500)).await {
            Ok(v) => println!("finished: {v}"),
            Err(d) => println!("timed out after {}ms", d.as_millis()),
        }

        match timeout(slow_job(), Duration::from_secs(5)).await {
            Ok(v) => println!("finished: {v}"),
            Err(d) => println!("timed out after {}ms", d.as_millis()),
        }
    });
}
