use super::key_expansion::key_expansion;
use crate::utils::constants::{NB_ROUND, S_BOX};
use crate::utils::multiply::{MULTIPLICATION_BY_2, MULTIPLICATION_BY_3};
use crate::utils::transform::{plain_to_square, square_to_plain};
use crate::utils::types::{Block, ExpendedKey, State, Word};

/// Encrypt a plain text block using the provided key.
pub fn encrypt(plain_text: &Block, key: &Block) -> Block {
    let mut state = plain_to_square(plain_text);
    transform_state(&mut state, key, NB_ROUND);
    square_to_plain(&state)
}

pub fn transform_state(state: &mut State, key: &Block, rounds: usize) {
    let expended_key = key_expansion(key);
    add_round_key(state, &key_for_round(expended_key, 0));
    for i in 1..rounds {
        sub_bytes(state);
        shift_rows(state);
        mix_columns(state);
        add_round_key(state, &key_for_round(expended_key, i));
    }
    sub_bytes(state);
    shift_rows(state);
    add_round_key(state, &key_for_round(expended_key, rounds));
}

fn key_for_round(expended_key: ExpendedKey, round: usize) -> [Word; 4] {
    [
        expended_key[round * 4],
        expended_key[round * 4 + 1],
        expended_key[round * 4 + 2],
        expended_key[round * 4 + 3],
    ]
}

fn shift_rows(state: &mut State) {
    let mut tmp;
    // second row
    tmp = state[1][0];
    for i in 0..3 {
        state[1][i] = state[1][i + 1];
    }
    state[1][3] = tmp;

    // third row
    tmp = state[2][0];
    state[2][0] = state[2][2];
    state[2][2] = tmp;
    tmp = state[2][1];
    state[2][1] = state[2][3];
    state[2][3] = tmp;

    // fourth row
    tmp = state[3][3];
    for i in (1..4).rev() {
        state[3][i] = state[3][i - 1];
    }
    state[3][0] = tmp;
}

fn mix_columns(state: &mut State) {
    for i in 0..4 {
        let (a0, a1, a2, a3) = (state[0][i], state[1][i], state[2][i], state[3][i]);
        state[0][i] = MULTIPLICATION_BY_2[a0 as usize] ^ MULTIPLICATION_BY_3[a1 as usize] ^ a2 ^ a3;
        state[1][i] = a0 ^ MULTIPLICATION_BY_2[a1 as usize] ^ MULTIPLICATION_BY_3[a2 as usize] ^ a3;
        state[2][i] = a0 ^ a1 ^ MULTIPLICATION_BY_2[a2 as usize] ^ MULTIPLICATION_BY_3[a3 as usize];
        state[3][i] = MULTIPLICATION_BY_3[a0 as usize] ^ a1 ^ a2 ^ MULTIPLICATION_BY_2[a3 as usize];
    }
}

fn sub_bytes(state: &mut State) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = S_BOX[state[i][j] as usize];
        }
    }
}
fn add_round_key(state: &mut State, key: &State) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] ^= key[j][i]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_state() {
        let key = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut state: State = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];

        transform_state(&mut state, &key, NB_ROUND);

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

        assert_eq!(
            ciphertext,
            [10, 148, 11, 181, 65, 110, 240, 69, 241, 195, 148, 88, 198, 83, 234, 90]
        );
    }
    #[test]
    fn test_shift_rows() {
        let mut dummy_state: State = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
        shift_rows(&mut dummy_state);
        assert_eq!(
            dummy_state,
            [[0, 1, 2, 3], [5, 6, 7, 4], [10, 11, 8, 9], [15, 12, 13, 14]]
        );
    }

    #[test]
    fn test_mix_columns() {
        let mut dummy_state: State = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];

        mix_columns(&mut dummy_state);

        assert_eq!(
            dummy_state,
            [
                [8, 9, 10, 11],
                [28, 29, 30, 31],
                [0, 1, 2, 3],
                [20, 21, 22, 23]
            ]
        )
    }
    #[test]
    fn test_sub_bytes() {
        let mut dummy_state: State = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
        sub_bytes(&mut dummy_state);
        assert_eq!(
            dummy_state,
            [
                [99, 124, 119, 123],
                [242, 107, 111, 197],
                [48, 1, 103, 43],
                [254, 215, 171, 118]
            ]
        )
    }
    #[test]
    fn test_add_round_key() {
        let mut dummy_state: State = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
        let dummy_key: State = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
        add_round_key(&mut dummy_state, &dummy_key);
        assert_eq!(
            dummy_state,
            [
                [0, 5, 10, 15],
                [5, 0, 15, 10],
                [10, 15, 0, 5],
                [15, 10, 5, 0]
            ]
        )
    }
}
