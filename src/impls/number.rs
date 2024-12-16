use std::iter::repeat_with;

use crate::Unreal;
use rand::{RngCore, seq::SliceRandom};

impl<R: RngCore> Unreal<R> {
    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    pub fn hex(&mut self, bits: usize) -> String {
        let digits = bits / 4;
        "0x".chars()
            .chain(
                repeat_with(|| {
                    *b"0123456789abcdef"
                        .choose(self)
                        .expect("list of hex digits should not be empty")
                        as char
                })
                .take(digits),
            )
            .collect()
    }
}
