use crate::utils::types::State;

pub fn add_round_key(state: &mut State, key: &State) {
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
