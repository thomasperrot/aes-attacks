use crate::utils::constants::REVERSED_S_BOX;
use crate::utils::multiply::{MULTIPLICATION_BY_2, MULTIPLICATION_BY_3};
use crate::utils::types::State;

pub fn get_all_possible_keys<'a>(
    normal_state: &State,
    faulty_state: &State,
) -> Vec<[&'a Vec<u8>; 0x10]> {
    let d1_equations = compute_first_column(normal_state, faulty_state);
    let d2_equations = compute_second_column(normal_state, faulty_state);
    let d3_equations = compute_third_column(normal_state, faulty_state);
    let d4_equations = compute_fourth_column(normal_state, faulty_state);
    let mut equations = Vec::new();
    for eq_1 in d1_equations.iter() {
        for eq_2 in d2_equations.iter() {
            for eq_3 in d3_equations.iter() {
                for eq_4 in d4_equations.iter() {
                    equations.push(get_equation(eq_1, eq_2, eq_3, eq_4));
                }
            }
        }
    }
    equations
}

#[rustfmt::skip]
fn get_equation<'a>(eq_1: &'a[Vec<u8>; 4], eq_2: &'a[Vec<u8>; 4], eq_3: &'a[Vec<u8>; 4], eq_4: &'a[Vec<u8>; 4]) -> [&'a Vec<u8>; 0x10] {
    [
        &eq_1[0], &eq_2[1], &eq_3[2], &eq_4[3],
        &eq_2[0], &eq_3[1], &eq_4[2], &eq_1[3],
        &eq_3[0], &eq_4[1], &eq_1[2], &eq_2[3],
        &eq_4[0], &eq_1[1], &eq_2[2], &eq_3[3],
    ]
}

fn compute_first_column(normal_state: &State, faulty_state: &State) -> Vec<[Vec<u8>; 4]> {
    let mut equations = Vec::new();
    for d in 1u8..=0xff {
        let mut potential_d_1: Vec<u8> = Vec::new();
        let mut potential_d_2: Vec<u8> = Vec::new();
        let mut potential_d_3: Vec<u8> = Vec::new();
        let mut potential_d_4: Vec<u8> = Vec::new();
        // x1
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[0][0] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[0][0] ^ k) as usize];
            if MULTIPLICATION_BY_2[d as usize] == diff {
                potential_d_1.push(k);
            }
        }
        if potential_d_1.is_empty() {
            continue;
        }
        // x14
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[1][3] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[1][3] ^ k) as usize];
            if d == diff {
                potential_d_2.push(k);
            }
        }
        if potential_d_2.is_empty() {
            continue;
        }
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[2][2] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[2][2] ^ k) as usize];
            if d == diff {
                potential_d_3.push(k);
            }
        }
        if potential_d_3.is_empty() {
            continue;
        }
        // x8
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[3][1] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[3][1] ^ k) as usize];
            if MULTIPLICATION_BY_3[d as usize] == diff {
                potential_d_4.push(k);
            }
        }
        if potential_d_4.is_empty() {
            continue;
        }
        equations.push([potential_d_1, potential_d_2, potential_d_3, potential_d_4]);
    }
    equations
}

fn compute_second_column(normal_state: &State, faulty_state: &State) -> Vec<[Vec<u8>; 4]> {
    let mut equations = Vec::new();
    for d in 1u8..=0xff {
        let mut potential_d_1: Vec<u8> = Vec::new();
        let mut potential_d_2: Vec<u8> = Vec::new();
        let mut potential_d_3: Vec<u8> = Vec::new();
        let mut potential_d_4: Vec<u8> = Vec::new();
        // x5
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[0][1] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[0][1] ^ k) as usize];
            if d == diff {
                potential_d_1.push(k);
            }
        }
        if potential_d_1.is_empty() {
            continue;
        }
        // x2
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[1][0] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[1][0] ^ k) as usize];
            if d == diff {
                potential_d_2.push(k);
            }
        }
        if potential_d_2.is_empty() {
            continue;
        }
        // x15
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[2][3] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[2][3] ^ k) as usize];
            if MULTIPLICATION_BY_3[d as usize] == diff {
                potential_d_3.push(k);
            }
        }
        if potential_d_3.is_empty() {
            continue;
        }
        // x12
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[3][2] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[3][2] ^ k) as usize];
            if MULTIPLICATION_BY_2[d as usize] == diff {
                potential_d_4.push(k);
            }
        }
        if potential_d_4.is_empty() {
            continue;
        }
        equations.push([potential_d_1, potential_d_2, potential_d_3, potential_d_4]);
    }
    equations
}

fn compute_third_column(normal_state: &State, faulty_state: &State) -> Vec<[Vec<u8>; 4]> {
    let mut equations = Vec::new();
    for d in 1u8..0xff {
        let mut potential_d_1: Vec<u8> = Vec::new();
        let mut potential_d_2: Vec<u8> = Vec::new();
        let mut potential_d_3: Vec<u8> = Vec::new();
        let mut potential_d_4: Vec<u8> = Vec::new();
        // x9
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[0][2] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[0][2] ^ k) as usize];
            if d == diff {
                potential_d_1.push(k);
            }
        }
        if potential_d_1.is_empty() {
            continue;
        }
        // x6
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[1][1] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[1][1] ^ k) as usize];
            if MULTIPLICATION_BY_3[d as usize] == diff {
                potential_d_2.push(k);
            }
        }
        if potential_d_2.is_empty() {
            continue;
        }
        // x3
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[2][0] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[2][0] ^ k) as usize];
            if MULTIPLICATION_BY_2[d as usize] == diff {
                potential_d_3.push(k);
            }
        }
        if potential_d_3.is_empty() {
            continue;
        }
        // x16
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[3][3] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[3][3] ^ k) as usize];
            if d == diff {
                potential_d_4.push(k);
            }
        }
        if potential_d_4.is_empty() {
            continue;
        }
        equations.push([potential_d_1, potential_d_2, potential_d_3, potential_d_4]);
    }
    equations
}

fn compute_fourth_column(normal_state: &State, faulty_state: &State) -> Vec<[Vec<u8>; 4]> {
    let mut equations = Vec::new();
    for d in 1u8..0xff {
        let mut potential_d_1: Vec<u8> = Vec::new();
        let mut potential_d_2: Vec<u8> = Vec::new();
        let mut potential_d_3: Vec<u8> = Vec::new();
        let mut potential_d_4: Vec<u8> = Vec::new();
        // x13
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[0][3] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[0][3] ^ k) as usize];
            if MULTIPLICATION_BY_3[d as usize] == diff {
                potential_d_1.push(k);
            }
        }
        if potential_d_1.is_empty() {
            continue;
        }
        // x10
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[1][2] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[1][2] ^ k) as usize];
            if MULTIPLICATION_BY_2[d as usize] == diff {
                potential_d_2.push(k);
            }
        }
        if potential_d_2.is_empty() {
            continue;
        }
        // x7
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[2][1] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[2][1] ^ k) as usize];
            if d == diff {
                potential_d_3.push(k);
            }
        }
        if potential_d_3.is_empty() {
            continue;
        }
        // x4
        for k in 0..=0xff {
            let diff = REVERSED_S_BOX[(normal_state[3][0] ^ k) as usize]
                ^ REVERSED_S_BOX[(faulty_state[3][0] ^ k) as usize];
            if d == diff {
                potential_d_4.push(k);
            }
        }
        if potential_d_4.is_empty() {
            continue;
        }
        equations.push([potential_d_1, potential_d_2, potential_d_3, potential_d_4]);
    }
    equations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_first_column() {
        let last_key = [
            162, 79, 213, 133, 38, 231, 209, 187, 72, 60, 127, 50, 147, 178, 71, 65,
        ];
        let normal_state = [
            [129, 189, 114, 129],
            [214, 22, 185, 139],
            [205, 251, 187, 91],
            [195, 141, 136, 233],
        ];
        let faulty_state = [
            [239, 99, 211, 112],
            [249, 1, 73, 230],
            [53, 135, 78, 136],
            [8, 184, 139, 126],
        ];
        let equations = compute_first_column(&normal_state, &faulty_state);
        let mut found = false;
        for eq in equations {
            if eq[0].contains(&last_key[0])
                && eq[1].contains(&last_key[7])
                && eq[2].contains(&last_key[10])
                && eq[3].contains(&last_key[13])
            {
                found = true;
            }
        }
        assert_eq!(found, true);
    }
}
