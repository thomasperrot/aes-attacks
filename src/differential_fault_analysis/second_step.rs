use crate::utils::constants::{LOOKUP_TABLE, REVERSED_S_BOX, S_BOX};
use crate::utils::multiply::{
    MULTIPLICATION_BY_11, MULTIPLICATION_BY_13, MULTIPLICATION_BY_14, MULTIPLICATION_BY_9,
};
use crate::utils::types::{Block, State};

fn compute_second_step_1(normal_state: State, faulty_state: State, key: Block) -> u8 {
    compute_second_step_1_for_state(normal_state, key)
        ^ compute_second_step_1_for_state(faulty_state, key)
}
fn compute_second_step_2(normal_state: State, faulty_state: State, key: Block) -> u8 {
    compute_second_step_2_for_state(normal_state, key)
        ^ compute_second_step_2_for_state(faulty_state, key)
}
fn compute_second_step_3(normal_state: State, faulty_state: State, key: Block) -> u8 {
    compute_second_step_3_for_state(normal_state, key)
        ^ compute_second_step_3_for_state(faulty_state, key)
}
fn compute_second_step_4(normal_state: State, faulty_state: State, key: Block) -> u8 {
    compute_second_step_4_for_state(normal_state, key)
        ^ compute_second_step_4_for_state(faulty_state, key)
}

fn compute_second_step_1_for_state(state: State, key: Block) -> u8 {
    let (a0, a1, a2, a3);
    {
        let a00 = REVERSED_S_BOX[state[0][0] ^ key[0]];
        let a01 = key[0] ^ S_BOX[key[13] ^ key[9]] ^ LOOKUP_TABLE[10];
        a0 = MULTIPLICATION_BY_14[a00 ^ a01];
    }
    {
        let a10 = REVERSED_S_BOX[state[1][3] ^ key[13]];
        let a11 = key[1] ^ S_BOX[key[14] ^ key[10]];
        a1 = MULTIPLICATION_BY_11[a10 ^ a11];
    }
    {
        let a20 = REVERSED_S_BOX[state[2][2] ^ key[10]];
        let a21 = key[2] ^ S_BOX[key[15] ^ key[11]];
        a2 = MULTIPLICATION_BY_13[a20 ^ a21];
    }
    {
        let a30 = REVERSED_S_BOX[state[3][1] ^ key[7]];
        let a31 = key[3] ^ S_BOX[key[12] ^ key[8]];
        a3 = MULTIPLICATION_BY_9[a30 ^ a31];
    }
    REVERSED_S_BOX[a0 ^ a1 ^ a2 ^ a3]
}

fn compute_second_step_2_for_state(state: State, key: Block) -> u8 {
    let (a0, a1, a2, a3);
    {
        let a00 = REVERSED_S_BOX[state[0][3] ^ key[12]];
        let a01 = key[12] ^ key[8];
        a0 = MULTIPLICATION_BY_9[a00 ^ a01];
    }
    {
        let a10 = REVERSED_S_BOX[state[1][2] ^ key[9]];
        let a11 = key[9] ^ key[13];
        a1 = MULTIPLICATION_BY_14[a10 ^ a11];
    }
    {
        let a20 = REVERSED_S_BOX[state[2][1] ^ key[6]];
        let a21 = key[14] ^ key[10];
        a2 = MULTIPLICATION_BY_11[a20 ^ a21];
    }
    {
        let a30 = REVERSED_S_BOX[state[3][0] ^ key[3]];
        let a31 = key[15] ^ key[11];
        a3 = MULTIPLICATION_BY_13[a30 ^ a31];
    }
    REVERSED_S_BOX[a0 ^ a1 ^ a2 ^ a3]
}

fn compute_second_step_3_for_state(state: State, key: Block) -> u8 {
    let (a0, a1, a2, a3);
    {
        let a00 = REVERSED_S_BOX[state[0][2] ^ key[8]];
        let a01 = key[8] ^ key[4];
        a0 = MULTIPLICATION_BY_13[a00 ^ a01];
    }
    {
        let a10 = REVERSED_S_BOX[state[1][1] ^ key[5]];
        let a11 = key[9] ^ key[5];
        a1 = MULTIPLICATION_BY_9[a10 ^ a11];
    }
    {
        let a20 = REVERSED_S_BOX[state[2][0] ^ key[2]];
        let a21 = key[10] ^ key[6];
        a2 = MULTIPLICATION_BY_14[a20 ^ a21];
    }
    {
        let a30 = REVERSED_S_BOX[state[3][3] ^ key[15]];
        let a31 = key[11] ^ key[7];
        a3 = MULTIPLICATION_BY_11[a30 ^ a31];
    }
    REVERSED_S_BOX[a0 ^ a1 ^ a2 ^ a3]
}

fn compute_second_step_4_for_state(state: State, key: Block) -> u8 {
    let (a0, a1, a2, a3);
    {
        let a00 = REVERSED_S_BOX[state[0][1] ^ key[4]];
        let a01 = key[4] ^ key[0];
        a0 = MULTIPLICATION_BY_11[a00 ^ a01];
    }
    {
        let a10 = REVERSED_S_BOX[state[1][0] ^ key[1]];
        let a11 = key[5] ^ key[1];
        a1 = MULTIPLICATION_BY_13[a10 ^ a11];
    }
    {
        let a20 = REVERSED_S_BOX[state[2][3] ^ key[14]];
        let a21 = key[6] ^ key[2];
        a2 = MULTIPLICATION_BY_9[a20 ^ a21];
    }
    {
        let a30 = REVERSED_S_BOX[state[3][2] ^ key[11]];
        let a31 = key[7] ^ key[3];
        a3 = MULTIPLICATION_BY_14[a30 ^ a31];
    }
    REVERSED_S_BOX[a0 ^ a1 ^ a2 ^ a3]
}
