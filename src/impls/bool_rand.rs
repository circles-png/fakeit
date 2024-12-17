use rand::{Rng, RngCore};

use crate::Unreal;

/// Generate random boolean values.
impl<R: RngCore> Unreal<R> {
    /// Return a random boolean value.
    pub fn bool(&mut self) -> bool {
        self.r#gen()
    }
}
