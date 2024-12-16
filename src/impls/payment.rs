use std::iter::repeat_with;

use chrono::{Datelike, Local};
use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

use crate::Unreal;
use crate::data::payment::CARD;

impl<R: RngCore> Unreal<R> {
    fn card(&mut self) -> (&'static str, &'static [&'static str]) {
        *CARD.choose(self).expect("CARDS should not be empty")
    }

    pub fn card_type(&mut self) -> &str {
        self.card().0
    }

    pub fn credit_card_number(&mut self) -> String {
        self.credit_card_number_inner().take(16).collect()
    }

    fn credit_card_number_inner(&mut self) -> impl Iterator<Item = char> {
        let starts = Self::card(self).1;
        let start =
            *SliceRandom::choose(starts, self).expect("all cards should have starting digits");
        start
            .chars()
            .chain(repeat_with(|| (b'0' + self.gen_range(0..=9)) as char))
    }

    pub fn credit_card_luhn_number(&mut self) -> String {
        let number: String = self.credit_card_number_inner().take(15).collect();
        let check_digit = ((10
            - (number
                .chars()
                .enumerate()
                .map(|(index, digit)| {
                    let digit = digit as u8 - b'0';
                    u32::from(if index % 2 == 0 {
                        (digit * 2) % 9
                    } else {
                        digit
                    })
                })
                .sum::<u32>()
                % 10))
            % 10)
            .to_string();
        number + &check_digit
    }

    pub fn credit_card_exp(&mut self) -> String {
        let year = (Local::now().year() + self.gen_range(1..=10)).to_string();
        let month = self.month();
        format!("{:0>2}/{}", month, &year[year.len() - 2..])
    }

    pub fn credit_card_cvv(&mut self) -> String {
        self.numbers(0..=999, 3)
    }
}
