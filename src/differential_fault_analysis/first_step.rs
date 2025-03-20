use crate::utils::constants::REVERSED_S_BOX;
use crate::utils::multiply::{MULTIPLICATION_BY_2, MULTIPLICATION_BY_3};
use crate::utils::types::{Block, State};
use arr_macro::arr;


pub fn get_all_possible_keys(normal_state: State, faulty_state: State) {
    let d1_equations = compute_first_column(normal_state, faulty_state);
    let d2_equations = compute_second_column(normal_state, faulty_state);
    let d3_equations = compute_third_column(normal_state, faulty_state);
    let d4_equations = compute_fourth_column(normal_state, faulty_state);
}

fn get_keys(equations: [[u8;0x100];16]) -> Vec<Block> {
    let keys: Vec<Block> = Vec::new();
    equations.
}

/// TODO: skip irrelevant d when previous iterations did not find it

fn compute_first_column(normal_state: State, faulty_state: State) -> Vec<[Vec<u8>; 4]>{
    // x1
    let mut potential_d_1: [Vec<u8>; 0x100] = arr![Vec::new(); 0x100];
    let mut potential_d_2: [Vec<u8>; 0x100] = arr![Vec::new(); 0x100];
    let mut potential_d_3: [Vec<u8>; 0x100] = arr![Vec::new(); 0x100];
    let mut potential_d_4: [Vec<u8>; 0x100] = arr![Vec::new(); 0x100];
    let mut equations = Vec::new();
    for d in 1..0x100 {
        for k in 0..=0xff{
            let diff = REVERSED_S_BOX[normal_state[0][0] ^ k]  ^ REVERSED_S_BOX[faulty_state[0][0] ^ k];
            if MULTIPLICATION_BY_2[d] == diff {
                potential_d_1[d].push(k);
            }
        }
    }
    // x14
    for d in 1..0x100 {
        for k in 0..=0xff{
            let diff = REVERSED_S_BOX[normal_state[1][3] ^ k]  ^ REVERSED_S_BOX[faulty_state[1][3] ^ k];
            if d == diff {
                potential_d_2[d].push(k);
            }
        }
    }
    // x11
    for d in 1..0x100 {
        for k in 0..=0xff{
            let diff = REVERSED_S_BOX[normal_state[2][2] ^ k]  ^ REVERSED_S_BOX[faulty_state[2][2] ^ k];
            if d == diff {
                potential_d_3[d].push(k);
            }
        }
    }
    // x8
    for d in 1..0x100 {
        for k in 0..=0xff{
            let diff = REVERSED_S_BOX[normal_state[3][1] ^ k]  ^ REVERSED_S_BOX[faulty_state[3][1] ^ k];
            if MULTIPLICATION_BY_3[d] == diff {
                potential_d_4[d].push(k);
            }
        }
    }

    let mut equations = Vec::new();
    for i in 0..0x100 {
        if !potential_d_1[i].is_empty() && !potential_d_2[i].is_empty() && !potential_d_3[i].is_empty() && !potential_d_4.is_empty() {
            equations.push([potential_d_1[i], potential_d_2[i], &potential_d_3[i], &potential_d_4[i]]);
        }
    }
    equations
}