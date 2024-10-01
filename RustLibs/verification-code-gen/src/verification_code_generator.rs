// Tic-Tac-Toe Service
//
// Provides 2-client game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use rand::Rng;

/// Provides a method to create a unique, 6-digit code. Such a code can, for instance, practical for end-users to utilize.
pub struct VerificationCodeGenerator {}

impl VerificationCodeGenerator {
    //

    /// Creates a unique, 6-digit code. Such a code can, for instance, practical for end-users to utilize.
    pub fn generate() -> String {
        let mut rng = rand::thread_rng();
        rng.gen_range(100_000..999_999).to_string()
    }
}
