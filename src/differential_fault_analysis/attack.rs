use crate::differential_fault_analysis::first_step::get_all_equations;
use crate::differential_fault_analysis::second_step::reduce_key_space;
use crate::utils::transform::plain_to_square;
use crate::utils::types::Block;

/// Crack an AES key using a differential fault analysis attack
pub fn attack(normal_cipher_text: &Block, faulty_cipher_text: &Block) -> Vec<Block> {
    let normal_state = plain_to_square(&normal_cipher_text);
    let faulty_state = plain_to_square(&faulty_cipher_text);
    println!("[ ] Computing all possible keys...");
    let equations = get_all_equations(&normal_state, &faulty_state);
    println!("[ ] Reducing key space...");
    let keys = reduce_key_space(&normal_state, &faulty_state, &equations);
    println!("[+] Finished !");
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
            41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41,
        ];
        let keys = attack(&normal_cipher_text, &faulty_cipher_text);
        // let keys = Vec::from( [
        //     [
        //         40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40,
        //     ],
        //     [
        //     41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41,
        // ],
        //  [
        //     42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        // ]]) ;
        for key in keys.iter() {
            println!("{key:x?}");
        }
        assert_eq!(keys.contains(&expected_key), true);
    }
}
