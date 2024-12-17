use chrono::DateTime;
use chrono::{Datelike, Local, NaiveDateTime};
use rand::{Rng, RngCore};

use crate::Unreal;
use crate::choose;
use crate::data::datetime::{ABR, FULL, OFFSET, TEXT};
use std::ops::{Bound, RangeBounds};

/// Generate random date and time data.
impl<R: RngCore> Unreal<R> {
    /// Return a random month from `1..=12`.
    pub fn month(&mut self) -> u32 {
        self.gen_range(1..=12)
    }

    /// Return a random day from `1..=28`.
    pub fn day(&mut self) -> u32 {
        self.gen_range(1..=28)
    }

    /// Return a random weekday from `0..=6`.
    pub fn week_day(&mut self) -> u32 {
        self.gen_range(0..=6)
    }

    /// Return a random year from `1980..=C` where C is the current year.
    pub fn year(&mut self) -> i32 {
        self.gen_range(1980..=Local::now().year())
    }

    /// Return a random hour from `0..=23`.
    pub fn hour(&mut self) -> u32 {
        self.gen_range(0..=23)
    }

    /// Return a random minute from `0..=59`.
    pub fn minute(&mut self) -> u32 {
        self.gen_range(0..=59)
    }

    /// Return a random second from `0..=59`.
    pub fn second(&mut self) -> u32 {
        self.gen_range(0..=59)
    }

    /// Return a random nanosecond from `0..=999_999_999`.
    pub fn nanosecond(&mut self) -> u32 {
        self.gen_range(0..=999_999_999)
    }

    choose! {
        /// Return a random timezone from the timezone data set.
        pub fn timezone(&mut self) from TEXT;
        /// Return a random extended timezone name from the full timezone data set.
        pub fn timezone_full(&mut self) from FULL;
        /// Return a random timezone abbreviation from the timezone abbreviation data set.
        pub fn timezone_abv(&mut self) from ABR;
        /// Return a random timezone offset from the timezone offset data set.
        pub fn timezone_offset(&mut self) from OFFSET;
    }

    /// Returns a random [`NaiveDateTime`] in the given range with nanosecond precision, or [`None`]
    /// if any of the bounds are out of range. See [`Self::datetime`] for more information on the
    /// allowed range.
    pub fn datetime_in_range(
        &mut self,
        range: impl RangeBounds<NaiveDateTime>,
    ) -> Option<NaiveDateTime> {
        let start_nanos = match range.start_bound() {
            Bound::Included(start) => start.and_utc().timestamp_nanos_opt()?,
            Bound::Excluded(start) => start.and_utc().timestamp_nanos_opt()? + 1,
            Bound::Unbounded => return None,
        };
        let end_nanos = match range.end_bound() {
            Bound::Included(end) => end.and_utc().timestamp_nanos_opt()? + 1,
            Bound::Excluded(end) => end.and_utc().timestamp_nanos_opt()?,
            Bound::Unbounded => return None,
        };

        Some(DateTime::from_timestamp_nanos(self.gen_range(start_nanos..end_nanos)).naive_local())
    }

    /// Returns a random [`NaiveDateTime`] from the UNIX epoch (January 1, 1970 at 00:00:00.000 UTC) to
    /// the current date and time.
    ///
    /// # Panics
    /// Panics if the current date and time is out of range (more than [`i64::MAX`] nanoseconds away
    /// from the UNIX epoch). This is before September 21, 1677 at 00:12:43.145224192 UTC or after
    /// April 11, 2262 at 23:47:16.854775807 UTC.
    pub fn datetime(&mut self) -> NaiveDateTime {
        DateTime::from_timestamp_nanos(
            self.gen_range(
                0..Local::now()
                    .timestamp_nanos_opt()
                    .expect("current date and time should be in range"),
            ),
        )
        .naive_local()
    }
}
