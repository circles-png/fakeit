use rand::RngCore;

use crate::Unreal;
use crate::choose;
use crate::data::hacker::{
    ABBREVIATION, ADJECTIVE, NOUN, VERB_INFINITIVE, VERB_PRESENT_PARTICIPLE,
};

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn abbreviation(&mut self) from ABBREVIATION;
        pub fn adjective(&mut self) from ADJECTIVE;
        pub fn noun(&mut self) from NOUN;
        pub fn verb_infinitive(&mut self) from VERB_INFINITIVE;
        pub fn verb_present_participle(&mut self) from VERB_PRESENT_PARTICIPLE;
    }

    pub fn phrase(&mut self) -> String {
        type F = fn(&str, &str, &str, &str, &str) -> String;
        let abbreviation = self.abbreviation();
        let adjective = self.adjective();
        let noun = self.noun();
        let verb_infinitive = self.verb_infinitive();
        let verb_present_participle = self.verb_present_participle();
        self.choose([
            (|abbreviation, adjective, noun, verb_infinitive, _verb_present_participle| format!("If we {verb_infinitive} the {noun}, we can get to the {abbreviation} {noun} through the {adjective} {abbreviation} {noun}!")) as F,
            (|abbreviation, adjective, noun, verb_infinitive, _verb_present_participle| format!("We need to {verb_infinitive} the {adjective} {abbreviation} {noun}!")) as F,
            (|abbreviation, adjective, noun, verb_infinitive, _verb_present_participle| format!("Try to {verb_infinitive} the {abbreviation} {noun}, maybe it will {verb_infinitive} the {adjective} {noun}!")) as F,
            (|abbreviation, adjective, noun, verb_infinitive, verb_present_participle| format!("You can't {verb_infinitive} the {noun} without {verb_present_participle} the {adjective} {abbreviation} {noun}!")) as F,
            (|abbreviation, adjective, noun, verb_infinitive, _verb_present_participle| format!("Use the {adjective} {abbreviation} {noun}, then you can {verb_infinitive} the {adjective} {noun}!")) as F,
            (|abbreviation, adjective, noun, verb_infinitive, _verb_present_participle| format!("The {abbreviation} {noun} is down, {verb_infinitive} the {adjective} {noun} so we can {verb_infinitive} the {abbreviation} {noun}!")) as F,
            (|abbreviation, adjective, noun, verb_infinitive, verb_present_participle| format!("{verb_present_participle} the {noun} won't do anything, we need to {verb_infinitive} the {adjective} {abbreviation} {noun}!")) as F,
            (|abbreviation, adjective, noun, verb_infinitive, _verb_present_participle| format!("I'll {verb_infinitive} the {adjective} {abbreviation} {noun}, that should {verb_infinitive} the {abbreviation} {noun}!")) as F,
        ])( abbreviation, adjective, noun, verb_infinitive, verb_present_participle)
    }
}
