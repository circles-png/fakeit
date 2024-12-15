use rand::RngCore;

use crate::Unreal;

impl<R: RngCore> Unreal<R> {
    #[must_use]
    pub fn phone(&mut self) -> String {
        #[allow(clippy::inconsistent_digit_grouping, reason = "this is a phone number")]
        self.numbers(100_000_0000..=999_999_9999, 9)
    }

    #[must_use]
    pub fn phone_formatted(&mut self) -> String {
        let numbers = [
            self.numbers(0..=999, 3),
            self.numbers(0..=999, 3),
            self.numbers(0..=9999, 4),
        ];
        self.choose([
            (|[a, b, c]| format!("{a}-{b}-{c}")) as fn([String; 3]) -> String,
            (|[a, b, c]| format!("({a}){b}-{c}")) as fn([String; 3]) -> String,
            (|[a, b, c]| format!("1-{a}-{b}-{c}")) as fn([String; 3]) -> String,
            (|[a, b, c]| format!("{a}.{b}.{c}")) as fn([String; 3]) -> String,
        ])(numbers)
    }

    #[must_use]
    pub fn email(&mut self) -> String {
        format!(
            "{}{}@{}.{}",
            self.first_name(),
            self.last_name(),
            self.last_name(),
            self.domain_suffix()
        )
        .to_lowercase()
    }
}
