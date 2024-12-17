use rand::RngCore;

use crate::Unreal;

/// Generate random contact data.
impl<R: RngCore> Unreal<R> {
    /// Return a random 10 digit phone number.
    pub fn phone(&mut self) -> String {
        self.digits(10)
    }

    /// Return a random 10 digit phone number in a random format, like `123-456-7890`.
    pub fn phone_formatted(&mut self) -> String {
        let numbers = [self.digits(3), self.digits(3), self.digits(4)];
        self.choose([
            (|[a, b, c]| format!("{a}-{b}-{c}")) as fn([String; 3]) -> String,
            (|[a, b, c]| format!("({a}){b}-{c}")) as fn([String; 3]) -> String,
            (|[a, b, c]| format!("1-{a}-{b}-{c}")) as fn([String; 3]) -> String,
            (|[a, b, c]| format!("{a}.{b}.{c}")) as fn([String; 3]) -> String,
        ])(numbers)
    }

    /// Return a random email address by concatenating
    /// - a random first name from [`Self::first_name`],
    /// - a random last name from [`Self::last_name`],
    /// - an `@` (U+0040 COMMERCIAL AT), and a
    /// - a random domain name from [`Self::domain_name`],
    ///
    /// then converting the result to lowercase.
    pub fn email(&mut self) -> String {
        format!(
            "{}{}@{}",
            self.first_name(),
            self.last_name(),
            self.domain_name(),
        )
        .to_lowercase()
    }
}
