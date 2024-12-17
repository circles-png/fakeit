use rand::RngCore;

use crate::data::log_level::{APACHE, GENERAL, SYSLOG};
use crate::{Unreal, choose};

/// Generate random log level data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random general log level from the general log level data set.
        pub fn general_log_level(&mut self) from GENERAL;
        /// Return a random syslog log level from the syslog log level data set.
        pub fn syslog_log_level(&mut self) from SYSLOG;
        /// Return a random Apache log level from the Apache log level data set.
        pub fn apache_log_level(&mut self) from APACHE;
    }
}
