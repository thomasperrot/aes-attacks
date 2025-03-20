use crate::utils::constants::REVERSED_S_BOX;
use crate::utils::multiply::MULTIPLICATION_BY_2;
use crate::utils::types::State;

pub fn get_all_possible_keys(normal_state: State, faulty_state: State) {

}

// fn compute_first_column(normal_state: State, faulty_state: State) {
//     let mut potential_d_1: [Vec<u8>; 0x100] = std::iter::repeat(vec![]).take(10).collect::<Vec<_>>();
//     for d in 1..0x100 {
//         for k in 0..=0xff{
//             let diff = REVERSED_S_BOX[normal_state[0][0] ^ k]  ^ REVERSED_S_BOX[faulty_state[0][0] ^ k];
//             if MULTIPLICATION_BY_2[d] = diff {
//                 potential_d_1[d].push(k);
//             }
//         }
//     }
// }