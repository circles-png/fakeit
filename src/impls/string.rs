use std::iter::repeat_with;

use rand::{Rng, RngCore};

use crate::Unreal;

/// Generate random string data.
impl<R: RngCore> Unreal<R> {
    /// Return a random lowercase ASCII letter.
    pub fn letter(&mut self) -> char {
        self.gen_range('a'..='z')
    }

    /// Return a string of random lowercase ASCII letters.
    pub fn letters(&mut self, length: usize) -> String {
        repeat_with(|| self.letter()).take(length).collect()
    }

    /// Return a random lowercase ASCII vowel. Vowels are `a`, `e`, `i`, `o`, and `u`.
    pub fn vowel(&mut self) -> char {
        self.choose(['a', 'e', 'i', 'o', 'u'])
    }

    /// Return a random ASCII digit.
    pub fn digit(&mut self) -> char {
        self.gen_range('0'..='9')
    }

    /// Return a string of random ASCII digits.
    ///
    /// # Panics
    /// Panics if the given length does not fit in a [`u32`].
    pub fn digits(&mut self, length: usize) -> String {
        self.numbers(
            0..10_usize.pow(u32::try_from(length).expect("given length should fit in a u32")),
            length,
        )
    }
}
