use rand::Rng;
use rand::RngCore;

use crate::Unreal;
use std::ops::Add;

/// Generate random person data.
impl<R: RngCore> Unreal<R> {
    /// Return a URL to a 300x300 pixel image of a person.
    pub fn image(&mut self) -> String {
        self.image_url(300, 300).add("/people")
    }

    /// Return a random 9 digit social security number.
    pub fn ssn(&mut self) -> String {
        self.digits(9)
    }

    /// Return a random gender, either `female` or `male`.
    pub fn gender(&mut self) -> &str {
        // are there more?
        if self.r#gen() { "female" } else { "male" }
    }
}
