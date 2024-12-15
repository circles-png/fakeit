use rand::RngCore;

use crate::Unreal;
use crate::choose;
use crate::data::company::{BS, BUZZWORDS, SUFFIX};

impl<R: RngCore> Unreal<R> {
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
        pub fn company_suffix(&mut self) from SUFFIX;
        pub fn buzzword(&mut self) from BUZZWORDS;
        pub fn bs(&mut self) from BS;
    }
}
