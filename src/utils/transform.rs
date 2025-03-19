use crate::utils::constants::BLOCK_SIZE;
use crate::utils::types::{Block, State};

pub fn plain_to_square(plain_text: &Block) -> State {
    let mut square = [[0u8; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            square[j][i] = plain_text[i * 4 + j];
        }
    }
    square
}

pub fn square_to_plain(square: &State) -> Block {
    let mut plain = [0u8; BLOCK_SIZE];
    for i in 0..4 {
        for j in 0..4 {
            plain[i * 4 + j] = square[j][i];
        }
    }
    plain
}

#[cfg(test)]
mod tests {
    use super::*;

    const DUMMY_PLAIN: Block = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    const DUMMY_SQUARE: State = [[0, 4, 8, 12], [1, 5, 9, 13], [2, 6, 10, 14], [3, 7, 11, 15]];

    #[test]
    fn test_plain_to_square() {
        assert_eq!(plain_to_square(&DUMMY_PLAIN), DUMMY_SQUARE);
    }

    #[test]
    fn test_square_to_plain() {
        assert_eq!(square_to_plain(&DUMMY_SQUARE), DUMMY_PLAIN);
    }
}
