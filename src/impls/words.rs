use std::iter::repeat_with;

use itertools::Itertools;
use rand::Rng;
use rand::RngCore;

use crate::Unreal;
use crate::choose;
use crate::data::lorem::WORD;

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn word(&mut self) from WORD;
    }

    pub fn sentence(&mut self, word_count: usize) -> String {
        (0..word_count)
            .map(|index| match index {
                0 => self
                    .word()
                    .char_indices()
                    .map(|(index, char)| {
                        if index == 0 {
                            char.to_ascii_uppercase()
                        } else {
                            char
                        }
                    })
                    .collect(),
                index if index == word_count - 1 => self.word().to_string() + ".",
                _ => self.word().to_string(),
            })
            .join(" ")
    }

    pub fn paragraph(
        &mut self,
        paragraphs: usize,
        sentences_per_paragraph: usize,
        words_per_sentence: usize,
        paragraph_separator: &str,
    ) -> String {
        repeat_with(|| {
            repeat_with(|| self.sentence(words_per_sentence))
                .take(sentences_per_paragraph)
                .join(" ")
        })
        .take(paragraphs)
        .join(paragraph_separator)
    }

    pub fn question(&mut self) -> String {
        let word_count = self.gen_range(3..=10);
        let sentence = self.sentence(word_count);
        sentence[..sentence.len() - 1].to_string() + "?"
    }

    pub fn quote(&mut self) -> String {
        let word_count = self.gen_range(3..=10);
        format!(
            r#""{}" - {} {}"#,
            self.sentence(word_count),
            self.first_name(),
            self.last_name()
        )
    }
}
