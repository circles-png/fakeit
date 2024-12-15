use std::iter::from_fn;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

use itertools::Itertools;
use rand::Rng;
use rand::RngCore;

use crate::Unreal;
use crate::choose;
use crate::data::internet::DOMAIN_SUFFIX;
use crate::data::internet::HTTP_METHOD;

impl<R: RngCore> Unreal<R> {
    pub fn domain_name(&mut self) -> String {
        format!(
            "{}{}.{}",
            self.descriptor().to_lowercase(),
            self.bs().to_lowercase(),
            self.domain_suffix()
        )
    }

    choose! {
        pub fn http_method(&mut self) from HTTP_METHOD;
        pub fn domain_suffix(&mut self) from DOMAIN_SUFFIX;
    }

    pub fn ipv4_address(&mut self) -> Ipv4Addr {
        Ipv4Addr::from_bits(self.r#gen())
    }

    pub fn ipv6_address(&mut self) -> Ipv6Addr {
        Ipv6Addr::from_bits(self.r#gen())
    }

    pub fn mac_address(&mut self) -> String {
        from_fn(|| Some(self.r#gen::<u8>()))
            .take(6)
            .map(|byte| format!("{byte:02x}"))
            .join(":")
    }

    pub fn username(&mut self) -> String {
        format!("{}{}", self.last_name(), self.numbers(0..=9999, 4),)
    }
}
