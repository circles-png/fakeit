#![feature(generic_arg_infer)]
#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::allow_attributes_without_reason
)]

use rand::{Error, Rng, RngCore, distributions::uniform::SampleRange};
mod address;
mod animal;
mod beer;
mod bool_rand;
mod colour;
mod company;
mod contact;
mod currency;
mod data;
mod datetime;
mod file;
mod hacker;
mod hipster;
mod image;
mod internet;
mod job;
mod language;
mod log_level;
mod name;
mod password;
mod payment;
mod person;
mod status_code;
mod user_agent;
mod uuid;
mod vehicle;
mod words;

pub struct Unreal<R: RngCore> {
    rng: R,
}

impl<R: RngCore> Unreal<R> {
    fn choose<T: Copy, const N: usize>(&mut self, array: [T; N]) -> T {
        array[self.gen_range(0..N)]
    }

    fn numbers(&mut self, range: impl SampleRange<usize>, min_width: usize) -> String {
        format!("{:0>min_width$}", self.gen_range(range))
    }
}

impl<R: RngCore> RngCore for Unreal<R> {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.rng.try_fill_bytes(dest)
    }
}

#[macro_export]
macro_rules! choose {
    (@method $visibility:vis fn $name:ident(&mut self) from $from:path;) => {
        #[must_use]
        $visibility fn $name(&mut self) -> &'static str {
            use rand::seq::SliceRandom;
            $from.choose(self).expect(concat!(stringify!($from), " should not be empty"))
        }
    };
    (@method $visibility:vis fn $name:ident(&mut self) -> $type:ty; from $from:path;) => {
        #[must_use]
        $visibility fn $name(&mut self) -> $type {
            use rand::seq::SliceRandom;
            $from.choose(self).expect(concat!(stringify!($from), " should not be empty"))
        }
    };
    {
        $(
            $visibility:vis fn $name:ident(&mut self) $(-> $type:ty;)? from $from:path;
        )+
    } => {
        $(
            $crate::choose!(@method $visibility fn $name(&mut self) $(-> $type;)? from $from;);
        )+
    };
}

#[macro_export]
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { ($crate::count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { $crate::count_tts!($($a)*) << 1 };
}

#[macro_export]
macro_rules! array_consts {
    {
        $(
            pub const $name:ident: [$type:ty; _] = [$($value:expr),* $(,)?];
        )+
    } => {
        $(
            pub const $name: [$type; $crate::count_tts!($($value)*)] = [$($value),*];
        )+
    };
}
