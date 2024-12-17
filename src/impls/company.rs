use rand::RngCore;

use crate::Unreal;
use crate::choose;
use crate::data::company::{BS, BUZZWORDS, SUFFIX};

/// Generate random company data.
impl<R: RngCore> Unreal<R> {
    /// Return a random company name.
    pub fn company(&mut self) -> String {
        self.choose([
            (|this: &mut Self| {
                format!(
                    "{}, {} and {}",
                    this.last_name(),
                    this.last_name(),
                    this.last_name()
                )
            }) as fn(&mut Self) -> String,
            (|this: &mut Self| format!("{}-{}", this.last_name(), this.last_name()))
                as fn(&mut Self) -> String,
            (|this: &mut Self| format!("{} {}", this.last_name(), this.company_suffix()))
                as fn(&mut Self) -> String,
        ])(self)
    }

    choose! {
        /// Return a random company suffix from the company suffix data set.
        pub fn company_suffix(&mut self) from SUFFIX;
        /// Return a random buzzword from the buzzword data set.
        pub fn buzzword(&mut self) from BUZZWORDS;
        /// Return a random word from the BS data set.
        pub fn bs(&mut self) from BS;
    }
}
