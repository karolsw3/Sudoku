//! Various utility functions.


use time::Duration;
use crypto::scrypt::{ScryptParams, scrypt_simple};


lazy_static! {
    /// The ideal "ideal" time `scrypt` – 250ms, which makes the hash time be between 250 and 500 milliseconds.
    pub static ref SCRYPT_MINIMUM_TIME: Duration = Duration::milliseconds(250);

    /// Induce the "ideal" parameters for `scrypt`, given a time the calculation must exceed, with the minimum of the recommended 2<sup>14</sup> rounds.
    ///
    /// That time is 250ms, in this case, which makes the hash time be between 250 and 500 milliseconds.
    ///
    /// Based on https://security.stackexchange.com/a/83382/124589.
    pub static ref SCRYPT_IDEAL_PARAMS: ScryptParams = {
        let starting_log2_n = 14;
        let block_size = 8;
        let parallelisation = 2;

        let duration = Duration::span(|| { let _ = scrypt_simple("microbenchmark", &ScryptParams::new(starting_log2_n, block_size, parallelisation)); });

        let log2_n_increase = ((SCRYPT_MINIMUM_TIME.num_milliseconds() as f64) / (duration.num_milliseconds() as f64)).log2().ceil() as u8;

        ScryptParams::new(starting_log2_n + log2_n_increase, block_size, parallelisation)
    };
}


include!(concat!(env!("OUT_DIR"), "/query.rs"));
