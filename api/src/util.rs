//! Various utility functions.


use std::iter;
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


/// Create a string consisting of `n` repetitions of `what`.
///
/// # Examples
///
/// ```
/// # use sudoku_backend::util::mul_str;
/// assert_eq!(&mul_str("Го! ", 3), "Го! Го! Го! ");
/// ```
pub fn mul_str(what: &str, n: usize) -> String {
    iter::repeat(what).take(n).collect()
}

/// Check if the specified board includes the specified skeleton.
///
/// # Examples
///
/// ```
/// # use sudoku_backend::util::board_includes;
/// assert!(board_includes("379625481625481397841937625958146273736852149214793568483569712597214836162378954",
///                        "3....5....25.81.........6......4..73.3....14..1.7......8...9.12.97........2..8.54"));
/// ```
pub fn board_includes(board: &str, skeleton: &str) -> bool {
    if board.len() == 9 * 9 && skeleton.len() == 9 * 9 {
        skeleton.chars().zip(board.chars()).all(|(s, b)| s == '.' || s == b)
    } else {
        false
    }
}

/// Get a random username.
///
/// TODO
pub fn random_username() -> &'static str {
    "benlo"
}
