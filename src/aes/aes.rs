use super::add_round_key::add_round_key;
use super::key_expansion::key_expansion;
use super::mix_columns::mix_columns;
use super::shift_rows::shift_rows;
use super::sub_bytes::sub_bytes;
use crate::utils::constants::NB_ROUND;
use crate::utils::transform::{plain_to_square, square_to_plain};
use crate::utils::types::{Block, ExpendedKey, State, Word};

/// Encrypt a plain text block using the provided key.
pub fn encrypt(plain_text: &Block, key: &Block) -> Block {
    let mut state = plain_to_square(plain_text);
    transform_state(&mut state, key);
    square_to_plain(&state)
}

pub fn transform_state(state: &mut State, key: &Block) {
    let expended_key = key_expansion(key);
    add_round_key(state, &key_for_round(expended_key, 0));
    for i in 1..NB_ROUND {
        sub_bytes(state);
        shift_rows(state);
        mix_columns(state);
        add_round_key(state, &key_for_round(expended_key, i));
    }
    sub_bytes(state);
    shift_rows(state);
    add_round_key(state, &key_for_round(expended_key, NB_ROUND));
}

fn key_for_round(expended_key: ExpendedKey, round: usize) -> [Word; 4] {
    [
        expended_key[round * 4],
        expended_key[round * 4 + 1],
        expended_key[round * 4 + 2],
        expended_key[round * 4 + 3],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_state() {
        let key = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut state: State = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];

        transform_state(&mut state, &key);

        assert_eq!(
            state,
            [
                [199, 20, 146, 169],
                [173, 121, 113, 79],
                [165, 152, 109, 151],
                [146, 177, 170, 234]
            ]
        );
    }

    #[test]
    fn test_encrypt() {
        let key = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let plain_text = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let ciphertext = encrypt(&plain_text, &key);

        assert_eq!(ciphertext, [10, 148, 11, 181, 65, 110, 240, 69, 241, 195, 148, 88, 198, 83, 234, 90]);
    }
}
