use serde::{Deserialize, Serialize};
use uint::construct_uint;

// Initial reward in bitcoins
pub const INITIAL_REWARD: u64 = 50;
// Halving interval in blocks
pub const HAVLING_INTERVAL: u64 = 210;
// Ideal block time in seconds
pub const IDEAL_BLOCK_TIME: u64 = 10;
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
]);
// Difficulty update interval in blocks
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;

construct_uint! {
    // Construct an unsigned 256-bit integer
    // consisting of 4 * 64-bit words

    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}

pub mod crypto;
pub mod errors;
pub mod sha256;
pub mod types;
pub mod util;