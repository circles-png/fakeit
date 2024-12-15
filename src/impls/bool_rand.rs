use rand::{Rng, RngCore};

use crate::Unreal;

impl<R: RngCore> Unreal<R> {
    pub fn bool(&mut self) -> bool {
        self.r#gen()
    }
}
