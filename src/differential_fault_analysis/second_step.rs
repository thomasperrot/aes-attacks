use crate::differential_fault_analysis::first_step::Equation;
use crate::utils::constants::{LOOKUP_TABLE, REVERSED_S_BOX, S_BOX};
use crate::utils::multiply::{
    MULTIPLICATION_BY_11, MULTIPLICATION_BY_13, MULTIPLICATION_BY_14, MULTIPLICATION_BY_2,
    MULTIPLICATION_BY_3, MULTIPLICATION_BY_9,
};
use crate::utils::types::{Block, State};

fn reduce_key_space(
    normal_state: &State,
    faulty_state: &State,
    equations: &Vec<Equation>,
) -> Vec<Block> {
    equations
        .iter()
        .flat_map(get_keys)
        .filter(|key| is_valid_guess(normal_state, faulty_state, key))
        .collect()
}

fn get_keys(equation: &Equation) -> Vec<Block> {
    let mut keys = Vec::new();
    for v0 in equation[0].iter() {
        for v1 in equation[1].iter() {
            for v2 in equation[2].iter() {
                for v3 in equation[3].iter() {
                    for v4 in equation[4].iter() {
                        for v5 in equation[5].iter() {
                            for v6 in equation[6].iter() {
                                for v7 in equation[7].iter() {
                                    for v8 in equation[8].iter() {
                                        for v9 in equation[9].iter() {
                                            for v10 in equation[10].iter() {
                                                for v11 in equation[11].iter() {
                                                    for v12 in equation[12].iter() {
                                                        for v13 in equation[13].iter() {
                                                            for v14 in equation[14].iter() {
                                                                for v15 in equation[15].iter() {
                                                                    keys.push([
                                                                        *v0, *v1, *v2, *v3, *v4,
                                                                        *v5, *v6, *v7, *v8, *v9,
                                                                        *v10, *v11, *v12, *v13,
                                                                        *v14, *v15,
                                                                    ]);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    keys
}

fn is_valid_guess(normal_state: &State, faulty_state: &State, key: &Block) -> bool {
    let fault = compute_second_step_2(normal_state, faulty_state, key);
    compute_second_step_3(normal_state, faulty_state, key) == fault
        && compute_second_step_1(normal_state, faulty_state, key)
            == MULTIPLICATION_BY_2[fault as usize]
        && compute_second_step_4(normal_state, faulty_state, key)
            == MULTIPLICATION_BY_3[fault as usize]
}

fn compute_second_step_1(normal_state: &State, faulty_state: &State, key: &Block) -> u8 {
    compute_second_step_1_for_state(normal_state, key)
        ^ compute_second_step_1_for_state(faulty_state, key)
}

fn compute_second_step_2(normal_state: &State, faulty_state: &State, key: &Block) -> u8 {
    compute_second_step_2_for_state(normal_state, key)
        ^ compute_second_step_2_for_state(faulty_state, key)
}

fn compute_second_step_3(normal_state: &State, faulty_state: &State, key: &Block) -> u8 {
    compute_second_step_3_for_state(normal_state, key)
        ^ compute_second_step_3_for_state(faulty_state, key)
}

fn compute_second_step_4(normal_state: &State, faulty_state: &State, key: &Block) -> u8 {
    compute_second_step_4_for_state(normal_state, key)
        ^ compute_second_step_4_for_state(faulty_state, key)
}

fn compute_second_step_1_for_state(state: &State, key: &Block) -> u8 {
    let (a0, a1, a2, a3);
    {
        let a00 = REVERSED_S_BOX[(state[0][0] ^ key[0]) as usize];
        let a01 = key[0] ^ S_BOX[(key[13] ^ key[9]) as usize] ^ LOOKUP_TABLE[10];
        a0 = MULTIPLICATION_BY_14[(a00 ^ a01) as usize];
    }
    {
        let a10 = REVERSED_S_BOX[(state[1][3] ^ key[13]) as usize];
        let a11 = key[1] ^ S_BOX[(key[14] ^ key[10]) as usize];
        a1 = MULTIPLICATION_BY_11[(a10 ^ a11) as usize];
    }
    {
        let a20 = REVERSED_S_BOX[(state[2][2] ^ key[10]) as usize];
        let a21 = key[2] ^ S_BOX[(key[15] ^ key[11]) as usize];
        a2 = MULTIPLICATION_BY_13[(a20 ^ a21) as usize];
    }
    {
        let a30 = REVERSED_S_BOX[(state[3][1] ^ key[7]) as usize];
        let a31 = key[3] ^ S_BOX[(key[12] ^ key[8]) as usize];
        a3 = MULTIPLICATION_BY_9[(a30 ^ a31) as usize];
    }
    REVERSED_S_BOX[(a0 ^ a1 ^ a2 ^ a3) as usize]
}

fn compute_second_step_2_for_state(state: &State, key: &Block) -> u8 {
    let (a0, a1, a2, a3);
    {
        let a00 = REVERSED_S_BOX[(state[0][3] ^ key[12]) as usize];
        let a01 = key[12] ^ key[8];
        a0 = MULTIPLICATION_BY_9[(a00 ^ a01) as usize];
    }
    {
        let a10 = REVERSED_S_BOX[(state[1][2] ^ key[9]) as usize];
        let a11 = key[9] ^ key[13];
        a1 = MULTIPLICATION_BY_14[(a10 ^ a11) as usize];
    }
    {
        let a20 = REVERSED_S_BOX[(state[2][1] ^ key[6]) as usize];
        let a21 = key[14] ^ key[10];
        a2 = MULTIPLICATION_BY_11[(a20 ^ a21) as usize];
    }
    {
        let a30 = REVERSED_S_BOX[(state[3][0] ^ key[3]) as usize];
        let a31 = key[15] ^ key[11];
        a3 = MULTIPLICATION_BY_13[(a30 ^ a31) as usize];
    }
    REVERSED_S_BOX[(a0 ^ a1 ^ a2 ^ a3) as usize]
}

fn compute_second_step_3_for_state(state: &State, key: &Block) -> u8 {
    let (a0, a1, a2, a3);
    {
        let a00 = REVERSED_S_BOX[(state[0][2] ^ key[8]) as usize];
        let a01 = key[8] ^ key[4];
        a0 = MULTIPLICATION_BY_13[(a00 ^ a01) as usize];
    }
    {
        let a10 = REVERSED_S_BOX[(state[1][1] ^ key[5]) as usize];
        let a11 = key[9] ^ key[5];
        a1 = MULTIPLICATION_BY_9[(a10 ^ a11) as usize];
    }
    {
        let a20 = REVERSED_S_BOX[(state[2][0] ^ key[2]) as usize];
        let a21 = key[10] ^ key[6];
        a2 = MULTIPLICATION_BY_14[(a20 ^ a21) as usize];
    }
    {
        let a30 = REVERSED_S_BOX[(state[3][3] ^ key[15]) as usize];
        let a31 = key[11] ^ key[7];
        a3 = MULTIPLICATION_BY_11[(a30 ^ a31) as usize];
    }
    REVERSED_S_BOX[(a0 ^ a1 ^ a2 ^ a3) as usize]
}

fn compute_second_step_4_for_state(state: &State, key: &Block) -> u8 {
    let (a0, a1, a2, a3);
    {
        let a00 = REVERSED_S_BOX[(state[0][1] ^ key[4]) as usize];
        let a01 = key[4] ^ key[0];
        a0 = MULTIPLICATION_BY_11[(a00 ^ a01) as usize];
    }
    {
        let a10 = REVERSED_S_BOX[(state[1][0] ^ key[1]) as usize];
        let a11 = key[5] ^ key[1];
        a1 = MULTIPLICATION_BY_13[(a10 ^ a11) as usize];
    }
    {
        let a20 = REVERSED_S_BOX[(state[2][3] ^ key[14]) as usize];
        let a21 = key[6] ^ key[2];
        a2 = MULTIPLICATION_BY_9[(a20 ^ a21) as usize];
    }
    {
        let a30 = REVERSED_S_BOX[(state[3][2] ^ key[11]) as usize];
        let a31 = key[7] ^ key[3];
        a3 = MULTIPLICATION_BY_14[(a30 ^ a31) as usize];
    }
    REVERSED_S_BOX[(a0 ^ a1 ^ a2 ^ a3) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: Block = [
        162, 79, 213, 133, 38, 231, 209, 187, 72, 60, 127, 50, 147, 178, 71, 65,
    ];
    const NORMAL_STATE: State = [
        [129, 189, 114, 129],
        [214, 22, 185, 139],
        [205, 251, 187, 91],
        [195, 141, 136, 233],
    ];
    const FAULTY_STATE: State = [
        [239, 99, 211, 112],
        [249, 1, 73, 230],
        [53, 135, 78, 136],
        [8, 184, 139, 126],
    ];
    #[test]
    fn test_is_valid_guess() {
        assert_eq!(is_valid_guess(&NORMAL_STATE, &FAULTY_STATE, &KEY), true);
    }

    #[test]
    fn test_is_valid_guess_invalid() {
        let key = [0; 16];
        assert_eq!(is_valid_guess(&NORMAL_STATE, &FAULTY_STATE, &key), false);
    }

    #[test]
    fn test_get_keys() {
        let equation: Equation = [
            Vec::from([0, 1]),
            Vec::from([2, 3]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
            Vec::from([0]),
        ];
        assert_eq!(
            get_keys(&equation),
            Vec::<Block>::from([
                [0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ])
        );
    }
}
