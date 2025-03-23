use crate::differential_fault_analysis::first_step::get_all_equations;
use crate::differential_fault_analysis::second_step::reduce_key_space;
use crate::utils::transform::plain_to_square;
use crate::utils::types::Block;
use std::cmp::min;
use std::thread;
use std::thread::JoinHandle;

/// Crack an AES key using a differential fault analysis attack
pub fn attack(
    normal_cipher_text: &Block,
    faulty_cipher_text: &Block,
    nb_threads: u8,
    nb_processes: u8,
    current_process: u8,
) -> Vec<Block> {
    let normal_state = plain_to_square(&normal_cipher_text);
    let faulty_state = plain_to_square(&faulty_cipher_text);
    let equations = get_all_equations(&normal_state, &faulty_state);

    let chunk_size = (equations.len() as f32 / nb_processes as f32).ceil() as usize;
    let equations = &equations[(chunk_size * current_process as usize)
        ..min(chunk_size * (current_process as usize + 1), equations.len())];

    let mut threads: Vec<JoinHandle<Vec<Block>>> = Vec::new();
    let chunk_size = (equations.len() as f32 / nb_threads as f32).ceil();
    for eq in equations.chunks(chunk_size as usize) {
        let cloned_equations = Vec::from(eq);
        let reduce_thread = thread::spawn(move || {
            reduce_key_space(&normal_state, &faulty_state, &cloned_equations)
        });
        threads.push(reduce_thread)
    }

    let mut keys: Vec<Block> = Vec::new();
    for t in threads {
        keys.extend(t.join().unwrap());
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    /// Test the whole attack. It takes 15min to complete.
    fn test_attack() {
        let normal_cipher_text = [
            129, 214, 205, 195, 189, 22, 251, 141, 114, 185, 187, 136, 129, 139, 91, 233,
        ];
        let faulty_cipher_text = [
            239, 249, 53, 8, 99, 1, 135, 184, 211, 73, 78, 139, 112, 230, 136, 126,
        ];
        let expected_key = [
            65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
        ];
        let keys = attack(&normal_cipher_text, &faulty_cipher_text, 5, 1, 0);
        assert_eq!(keys.contains(&expected_key), true);
    }
}
