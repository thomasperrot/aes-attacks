use super::attack::DeltaSet;
use crate::aes::transform_state;
use crate::utils::types::Block;
use hex;

pub trait Oracle {
    fn encrypt(&mut self, delta_set: &mut DeltaSet) -> ();
}

pub struct LocalOracle {
    pub key: Block,
}

impl Oracle for LocalOracle {
    fn encrypt(&mut self, delta_set: &mut DeltaSet) {
        for mut ds in delta_set.iter_mut() {
            transform_state(&mut ds, &self.key, 4);
        }
    }
}

pub struct InvalidOracle {}

impl Oracle for InvalidOracle {
    fn encrypt(&mut self, _: &mut DeltaSet) -> () {}
}
