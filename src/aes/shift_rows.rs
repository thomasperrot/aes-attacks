use crate::utils::types::State;

pub fn shift_rows(state: &mut State) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_rows() {
        let mut dummy_state: State = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
        shift_rows(&mut dummy_state);
        assert_eq!(
            dummy_state,
            [[0, 1, 2, 3], [5, 6, 7, 4], [10, 11, 8, 9], [15, 12, 13, 14]]
        );
    }
}
