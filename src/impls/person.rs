use rand::Rng;
use rand::RngCore;

use crate::Unreal;
use std::ops::Add;

impl<R: RngCore> Unreal<R> {
    pub fn image(&mut self) -> String {
        Self::image_url(300, 300).add("/people")
    }

    pub fn ssn(&mut self) -> String {
        #[allow(
            clippy::inconsistent_digit_grouping,
            reason = "this is a US social security number"
        )]
        self.numbers(0..=999_99_9999, 9)
    }

    pub fn gender(&mut self) -> &str {
        // are there more?
        if self.r#gen() { "female" } else { "male" }
    }
}
