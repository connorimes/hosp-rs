//! A module for utility functions and data.

use std::thread::sleep;
use std::time::Duration;
use super::HospResult;

/// The normal refresh interval for an ODROID Smart Power.
pub const REFRESH_INTERVAL_MS: u64 = 100;

/// A utility for retrying getter functions.
/// Given `h: HospDevice`, `op` should be one of `h.get_version()`, `h.get_status()`, or `h.get_data()`.
/// The `retries` parameter is the max number read attempts, with `delay_ms` milliseconds wait between attempts.
/// The function will panic if `retries` are exhausted or after `max_failures` consecutive I/O errors.
pub fn retry_get<O, H>(op: O, retries: u32, delay_ms: u64, max_failures: u32) -> H
where O: Fn() -> HospResult<Option<H>> {
    let mut failures = 0;
    let ms = Duration::from_millis(delay_ms);
    for _ in 1..retries {
        match op() {
            Ok(opt) => {
                // check if response is actually available
                if let Some(d) = opt {
                    return d;
                }
                // reset failure counter
                failures = 0;
            },
            Err(_) => {
                failures += 1;
                if failures == max_failures {
                    break;
                }
            }
        }
        sleep(ms);
    }
    panic!("Failed to read from ODROID Smart Power");
}
