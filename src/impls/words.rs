use std::iter::repeat_with;

use itertools::Itertools;
use rand::Rng;
use rand::RngCore;

use crate::Unreal;
use crate::choose;
use crate::data::lorem::WORD;

/// Generate random word data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random word from the lorem word data set.
        pub fn word(&mut self) from WORD;
    }

    /// Return a sentence of `word_count` words, each generated from [`Self::word`].
    ///
    /// The first letter is capitalised and the sentence ends with a `.` (U+002E FULL STOP).
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

    /// Return `paragraphs` paragraphs separated by `paragraph_separator`, each with
    /// `sentences_per_paragraph` sentences, each with `words_per_sentence` words.
    ///
    /// Sentences are separated by a ` ` (U+0020 SPACE), and are generated from [`Self::sentence`].
    pub fn paragraphs(
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

    /// Return a question of `3..=10` words, generated from [`Self::sentence`], but with
    /// a `?` (U+003F QUESTION MARK) instead of a `.` (U+002E FULL STOP) at the end.
    pub fn question(&mut self) -> String {
        let word_count = self.gen_range(3..=10);
        let sentence = self.sentence(word_count);
        sentence[..sentence.len() - 1].to_string() + "?"
    }

    /// Return a quote - a `3..=10` word sentence generated from [`Self::sentence`], surrounded by `""`, followed by
    /// ` - `, and a full name generated from [`Self::full_name`].
    pub fn quote(&mut self) -> String {
        let word_count = self.gen_range(3..=10);
        format!(r#""{}" - {}"#, self.sentence(word_count), self.full_name())
    }
}
