use chrono::NaiveDateTime;
use rand::{Rng, RngCore};
use uuid::Uuid;
use uuid::v1::Timestamp;

use crate::Unreal;

/// Generate random UUIDs.
impl<R: RngCore> Unreal<R> {
    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    /// Return a random version 1 UUID with a random MAC address. The result is hyphenated and lowercase.
    ///
    /// Uses [`Self::datetime`] and a random counter value for the timestamp.
    ///
    /// # Panics
    ///
    /// See [`Self::datetime`].
    pub fn uuid_v1(&mut self) -> String {
        let duration = self
            .datetime()
            .signed_duration_since(NaiveDateTime::UNIX_EPOCH)
            .to_std()
            .expect("date should be ahead of the unix epoch");
        Uuid::new_v1(
            Timestamp::from_unix_time(
                duration.as_secs(),
                duration.subsec_nanos(),
                self.r#gen(),
                128,
            ),
            &self.r#gen(),
        )
        .to_string()
    }

    /// Return a random version 4 UUID. The result is hyphenated and lowercase.
    pub fn uuid_v4(&mut self) -> String {
        Uuid::from_u128(self.r#gen()).to_string()
    }
}
