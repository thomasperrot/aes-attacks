use super::sub_bytes::S_BOX;
use crate::utils::constants::{EXPANDED_SIZE, NB_ROUND};
use crate::utils::types::{Block, ExpendedKey, Word};

const LOOKUP_TABLE: [u8; 0x100] = [
    141, 1, 2, 4, 8, 16, 32, 64, 128, 27, 54, 108, 216, 171, 77, 154, 47, 94, 188, 99, 198, 151,
    53, 106, 212, 179, 125, 250, 239, 197, 145, 57, 114, 228, 211, 189, 97, 194, 159, 37, 74, 148,
    51, 102, 204, 131, 29, 58, 116, 232, 203, 141, 1, 2, 4, 8, 16, 32, 64, 128, 27, 54, 108, 216,
    171, 77, 154, 47, 94, 188, 99, 198, 151, 53, 106, 212, 179, 125, 250, 239, 197, 145, 57, 114,
    228, 211, 189, 97, 194, 159, 37, 74, 148, 51, 102, 204, 131, 29, 58, 116, 232, 203, 141, 1, 2,
    4, 8, 16, 32, 64, 128, 27, 54, 108, 216, 171, 77, 154, 47, 94, 188, 99, 198, 151, 53, 106, 212,
    179, 125, 250, 239, 197, 145, 57, 114, 228, 211, 189, 97, 194, 159, 37, 74, 148, 51, 102, 204,
    131, 29, 58, 116, 232, 203, 141, 1, 2, 4, 8, 16, 32, 64, 128, 27, 54, 108, 216, 171, 77, 154,
    47, 94, 188, 99, 198, 151, 53, 106, 212, 179, 125, 250, 239, 197, 145, 57, 114, 228, 211, 189,
    97, 194, 159, 37, 74, 148, 51, 102, 204, 131, 29, 58, 116, 232, 203, 141, 1, 2, 4, 8, 16, 32,
    64, 128, 27, 54, 108, 216, 171, 77, 154, 47, 94, 188, 99, 198, 151, 53, 106, 212, 179, 125,
    250, 239, 197, 145, 57, 114, 228, 211, 189, 97, 194, 159, 37, 74, 148, 51, 102, 204, 131, 29,
    58, 116, 232, 203, 141,
];

pub fn key_expansion(key: &Block) -> ExpendedKey {
    let mut expanded_key: ExpendedKey = [[0, 0, 0, 0]; EXPANDED_SIZE];
    for i in 0..4 {
        for j in 0..4 {
            expanded_key[i][j] = key[i * 4 + j];
        }
    }

    for i in 1..NB_ROUND + 1 {
        let rot = rot_word(&expanded_key[4 * i - 1]);
        let sub = sub_word(&rot);
        let xored = xor(&sub, &expanded_key[4 * i - 4]);
        let first_column = xor(&xored, &r_con(i));
        expanded_key[4 * i] = first_column;
        for j in 0..3 {
            expanded_key[4 * i + j + 1] =
                xor(&expanded_key[4 * i + j], &expanded_key[4 * i + j - 3]);
        }
    }
    expanded_key
}

pub fn get_first_key(key: &Block, rounds: u8) -> Block {
    let mut key_state: [Word; 4] = [[0, 0, 0, 0]; 4];
    let mut previous_key_state: [Word; 4] = [[0, 0, 0, 0]; 4];
    for i in 0..4 {
        for j in 0..4 {
            key_state[i][j] = key[i * 4 + j]
        }
    }

    for i in (1..rounds).rev() {
        for j in 1..4 {
            previous_key_state[j] = xor(&key_state[j - 1], &key_state[j])
        }
        let rot = rot_word(&previous_key_state[3]);
        let sub = sub_word(&rot);
        let xored = xor(&key_state[0], &r_con(i as usize));
        previous_key_state[0] = xor(&xored, &sub);
        key_state = previous_key_state
    }
    let mut first_key = [0; 16];
    for i in 0..4 {
        for j in 0..4 {
            first_key[i * 4 + j] = key_state[i][j]
        }
    }
    first_key
}

fn rot_word(word: &Word) -> Word {
    [word[1], word[2], word[3], word[0]]
}

fn sub_word(word: &Word) -> Word {
    [
        S_BOX[word[0] as usize],
        S_BOX[word[1] as usize],
        S_BOX[word[2] as usize],
        S_BOX[word[3] as usize],
    ]
}

fn xor(w1: &Word, w2: &Word) -> Word {
    [w1[0] ^ w2[0], w1[1] ^ w2[1], w1[2] ^ w2[2], w1[3] ^ w2[3]]
}

fn r_con(n: usize) -> Word {
    [LOOKUP_TABLE[n], 0, 0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot_word() {
        let word: Word = [0, 1, 2, 3];
        assert_eq!(rot_word(&word), [1, 2, 3, 0]);
    }

    #[test]
    fn test_sub_word() {
        let word: Word = [0, 1, 2, 3];
        assert_eq!(sub_word(&word), [99, 124, 119, 123])
    }

    #[test]
    fn test_xor() {
        let new_word = xor(&[0, 1, 2, 3], &[0, 1, 2, 3]);
        assert_eq!(new_word, [0, 0, 0, 0])
    }

    #[test]
    fn test_r_con() {
        assert_eq!(r_con(0), [141, 0, 0, 0])
    }

    #[test]
    fn test_key_expansion() {
        let key = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        assert_eq!(
            key_expansion(&key),
            [
                [0, 1, 2, 3],
                [4, 5, 6, 7],
                [8, 9, 10, 11],
                [12, 13, 14, 15],
                [214, 170, 116, 253],
                [210, 175, 114, 250],
                [218, 166, 120, 241],
                [214, 171, 118, 254],
                [182, 146, 207, 11],
                [100, 61, 189, 241],
                [190, 155, 197, 0],
                [104, 48, 179, 254],
                [182, 255, 116, 78],
                [210, 194, 201, 191],
                [108, 89, 12, 191],
                [4, 105, 191, 65],
                [71, 247, 247, 188],
                [149, 53, 62, 3],
                [249, 108, 50, 188],
                [253, 5, 141, 253],
                [60, 170, 163, 232],
                [169, 159, 157, 235],
                [80, 243, 175, 87],
                [173, 246, 34, 170],
                [94, 57, 15, 125],
                [247, 166, 146, 150],
                [167, 85, 61, 193],
                [10, 163, 31, 107],
                [20, 249, 112, 26],
                [227, 95, 226, 140],
                [68, 10, 223, 77],
                [78, 169, 192, 38],
                [71, 67, 135, 53],
                [164, 28, 101, 185],
                [224, 22, 186, 244],
                [174, 191, 122, 210],
                [84, 153, 50, 209],
                [240, 133, 87, 104],
                [16, 147, 237, 156],
                [190, 44, 151, 78],
                [19, 17, 29, 127],
                [227, 148, 74, 23],
                [243, 7, 167, 139],
                [77, 43, 48, 197],
            ]
        )
    }

    #[test]
    fn test_get_first_key() {
        let key = [
            71, 247, 247, 188, 149, 53, 62, 3, 249, 108, 50, 188, 253, 5, 141, 253,
        ];
        assert_eq!(
            get_first_key(&key, 5),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }
}
