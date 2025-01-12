use std::iter::repeat_with;

use rand::{
    RngCore,
    seq::{IteratorRandom, SliceRandom},
};

use crate::Unreal;

/// Generate random password data.
impl<R: RngCore> Unreal<R> {
    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    /// Generate a random password `length` characters long with the given parameters.
    ///
    /// - `upper`: Include ASCII uppercase letters in the password.
    /// - `numeric`: Include ASCII digits in the password.
    /// - `special`: Include some of `!@#$%&*+-=?` in the password.
    pub fn password(&mut self, upper: bool, numeric: bool, special: bool, length: usize) -> String {
        repeat_with(|| {
            *[
                Some("abcdefghijklmnopqrstuvwxyz"),
                Some("ABCDEFGHIJKLMNOPQRSTUVWXYZ").filter(|_| upper),
                Some("0123456789").filter(|_| numeric),
                Some("!@#$%&*+-=?").filter(|_| special),
            ]
            .into_iter()
            .flatten()
            .choose(self)
            .expect("there should be at least one character set")
            .as_bytes()
            .choose(self)
            .expect("no character sets should be empty") as char
        })
        .take(length)
        .collect()
    }
}
