use crate::Unreal;
use rand::{Rng, RngCore};

/// Generate random number data.
impl<R: RngCore> Unreal<R> {
    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    /// Generate a hex string with the given number of bits. The string will be lowercase and
    /// prefixed with `0x`.
    pub fn hex(&mut self, bits: usize) -> String {
        format!("{:#x}", self.gen_range(0..(1 << bits)))
    }
}
