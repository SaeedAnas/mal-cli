use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::iter;

/// Generates a new base64-encoded SHA-256 PKCE code
/// # Panic
/// `len` needs to be a value between 48 and 128
pub fn new_challenge(len: usize) -> String {
    // Check whether the len in in between the valid length for a
    // PKCE code (43 chars - 128 chars)
    if len < 48 || len > 128 {
        panic!("len is not in between 48 and 128");
    }
    let mut rng = thread_rng();
    // needs to be url safe so we use Alphanumeric
    let challenge: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(len)
        .collect();
    challenge
}
