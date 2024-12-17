use std::iter::repeat_with;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

use itertools::Itertools;
use rand::Rng;
use rand::RngCore;

use crate::Unreal;
use crate::choose;
use crate::data::internet::DOMAIN_SUFFIX;
use crate::data::internet::HTTP_METHOD;

/// Generate random internet data.
impl<R: RngCore> Unreal<R> {
    /// Return a random domain name by concatenating
    /// - a random job descriptor from [`Self::descriptor`], converted to lowercase,
    /// - a random word from [`Self::bs`], converted to lowercase,
    /// - a `.` (U+002E FULL STOP), and
    /// - a random domain suffix from [`Self::domain_suffix`].
    pub fn domain_name(&mut self) -> String {
        format!(
            "{}{}.{}",
            self.descriptor().to_lowercase(),
            self.bs().to_lowercase(),
            self.domain_suffix()
        )
    }

    choose! {
        /// Return a random HTTP method from the HTTP method data set.
        pub fn http_method(&mut self) from HTTP_METHOD;
        /// Return a random domain suffix from the domain suffix data set.
        pub fn domain_suffix(&mut self) from DOMAIN_SUFFIX;
    }

    /// Return a random IPv4 address.
    pub fn ipv4_address(&mut self) -> Ipv4Addr {
        Ipv4Addr::from_bits(self.r#gen())
    }

    /// Return a random IPv6 address.
    pub fn ipv6_address(&mut self) -> Ipv6Addr {
        Ipv6Addr::from_bits(self.r#gen())
    }

    /// Return a random MAC address.
    pub fn mac_address(&mut self) -> String {
        repeat_with(|| self.r#gen::<u8>())
            .take(6)
            .map(|byte| format!("{byte:02x}"))
            .join(":")
    }

    /// Return a random username by concatenating a random last name from [`Self::last_name`] and
    /// four random digits.
    pub fn username(&mut self) -> String {
        format!("{}{}", self.last_name(), self.digits(4))
    }
}
