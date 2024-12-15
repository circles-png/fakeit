use rand::distributions::uniform::SampleRange;
use rand::{Rng, RngCore};

use crate::data::currency::UNITS;
use crate::{Unreal, choose};

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn unit(&mut self) -> &(&str, &str); from UNITS;
    }

    pub fn currency_short(&mut self) -> &str {
        self.unit().0
    }

    pub fn currency_long(&mut self) -> &str {
        self.unit().1
    }

    pub fn price(&mut self, range: impl SampleRange<f32>) -> f32 {
        (self.gen_range(range) * 100.).floor() / 100.
    }
}
