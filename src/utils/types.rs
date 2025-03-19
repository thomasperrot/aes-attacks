use super::constants::{BLOCK_SIZE, NB_ROUND};

pub type Block = [u8; BLOCK_SIZE];
pub type Word = [u8; 4];
pub type State = [Word; 4];
pub type ExpendedKey = [Word; 4 * (NB_ROUND + 1)];

