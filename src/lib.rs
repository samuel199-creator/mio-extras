//! Extra components for use with Mio.
#![deny(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate once_cell;
extern crate mio;
extern crate slab;

#[macro_use]
extern crate log;

pub mod channel;
pub mod timer;

// Conversion utilities
mod convert {
    use std::time::Duration;

    const NANOS_PER_MILLI: u32 = 1_000_000;
    const MILLIS_PER_SEC: u64 = 1_000;

    /// Convert a `Duration` to milliseconds, rounding up and saturating at
    /// `u64::MAX`.
    ///
    /// The saturating is fine because `u64::MAX` milliseconds are still many
    /// million years.
    pub fn millis(duration: Duration) -> u64 {
        // Round up.
        let millis = (duration.subsec_nanos() + NANOS_PER_MILLI - 1) / NANOS_PER_MILLI;
        duration
            .as_secs()
            .saturating_mul(MILLIS_PER_SEC)
            .saturating_add(u64::from(millis))
    }
}
