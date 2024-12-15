//! # unreal
//!
//! Fake data generator. Fork of the [fakeit](https://github.com/PumpkinSeed/fakeit) crate.

#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::allow_attributes_without_reason
)]

mod data;
mod impls;
mod macros;
mod unreal;
pub(crate) use macros::{array_consts, choose, count_tts};
pub use unreal::Unreal;
