use std::iter::from_fn;

use rand::{Rng, RngCore};

use crate::Unreal;

impl<R: RngCore> Unreal<R> {
    pub fn letter(&mut self) -> char {
        self.gen_range('a'..='z')
    }

    pub fn letters(&mut self, length: usize) -> String {
        from_fn(|| Some(self.letter())).take(length).collect()
    }

    pub fn vowel(&mut self) -> char {
        self.choose(['a', 'e', 'i', 'o', 'u'])
    }

    pub fn digit(&mut self) -> char {
        self.gen_range('0'..='9')
    }

    /// # Panics
    /// Panics if the given length does not fit in a [`u32`].
    pub fn digits(&mut self, length: usize) -> String {
        self.numbers(
            0..10_usize.pow(u32::try_from(length).expect("given length should fit in a u32")),
            length,
        )
    }
}
