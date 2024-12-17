use rand::RngCore;

use crate::Unreal;
use crate::choose;
use crate::data::hacker::{
    ABBREVIATION, ADJECTIVE, NOUN, VERB_INFINITIVE, VERB_PRESENT_PARTICIPLE,
};

/// Generate random 'hacker' data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random 'hacker' abbreviation from the 'hacker' abbreviation data set.
        pub fn abbreviation_hacker(&mut self) from ABBREVIATION;
        /// Return a random 'hacker' adjective from the 'hacker' adjective data set.
        pub fn adjective_hacker(&mut self) from ADJECTIVE;
        /// Return a random 'hacker' noun from the 'hacker' noun data set.
        pub fn noun_hacker(&mut self) from NOUN;
        /// Return a random 'hacker' infinitive verb from the 'hacker' infinitive verb data set.
        pub fn verb_infinitive_hacker(&mut self) from VERB_INFINITIVE;
        /// Return a random 'hacker' verb in present participle form from the 'hacker' present
        /// participle verb data set.
        pub fn verb_present_participle_hacker(&mut self) from VERB_PRESENT_PARTICIPLE;
    }

    /// Generate a random 'hacker' phrase.
    pub fn phrase(&mut self) -> String {
        type F = fn(&str, &str, &str, &str, &str) -> String;
        self.choose([
            (|ab, ad, n, vi, _vp| {
                format!("If we {vi} the {n}, we can get to the {ab} {n} through the {ad} {ab} {n}!")
            }) as F,
            (|ab, ad, n, vi, _vp| format!("We need to {vi} the {ad} {ab} {n}!")) as F,
            (|ab, ad, n, vi, _vp| {
                format!("Try to {vi} the {ab} {n}, maybe it will {vi} the {ad} {n}!")
            }) as F,
            (|ab, ad, n, vi, vp| format!("You can't {vi} the {n} without {vp} the {ad} {ab} {n}!"))
                as F,
            (|ab, ad, n, vi, _vp| format!("Use the {ad} {ab} {n}, then you can {vi} the {ad} {n}!"))
                as F,
            (|ab, ad, n, vi, _vp| {
                format!("The {ab} {n} is down, {vi} the {ad} {n} so we can {vi} the {ab} {n}!")
            }) as F,
            (|ab, ad, n, vi, vp| {
                format!("{vp} the {n} won't do anything, we need to {vi} the {ad} {ab} {n}!")
            }) as F,
            (|ab, ad, n, vi, _vp| {
                format!("I'll {vi} the {ad} {ab} {n}, that should {vi} the {ab} {n}!")
            }) as F,
        ])(
            self.abbreviation_hacker(),
            self.adjective_hacker(),
            self.noun_hacker(),
            self.verb_infinitive_hacker(),
            self.verb_present_participle_hacker(),
        )
    }
}
