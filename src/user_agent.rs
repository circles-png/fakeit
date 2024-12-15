use rand::Rng;
use rand::RngCore;
use rand::seq::SliceRandom;

use crate::Unreal;
use crate::data::computer::LINUX_PROCESSOR;
use crate::data::computer::MAC_PROCESSOR;
use crate::data::computer::WINDOWS_PLATFORM;

impl<R: RngCore> Unreal<R> {
    #[must_use]
    pub fn chrome(&mut self) -> String {
        let version = self.gen_range(531..=538);
        format!(
            "Mozilla/5.0 ({}) AppleWebKit/{} (KHTML, like Gecko) Chrome/{}.0.{}.0 Mobile Safari/{}",
            self.random_platform(),
            version,
            self.gen_range(36..=40),
            self.gen_range(800..=899),
            version
        )
    }

    #[must_use]
    pub fn firefox(&mut self) -> String {
        let date = format!("{}-{}-{}", self.year(), self.month(), self.day());
        let platform = self.choose([
            (|this: &mut Self| {
                format!(
                    "({}; en-US; rv:1.9.{}.20)",
                    this.windows_platform_token(),
                    this.gen_range(0..=3)
                )
            }) as fn(&mut Self) -> String,
            (|this: &mut Self| {
                format!(
                    "({}; rv:{}.0)",
                    this.linux_platform_token(),
                    this.gen_range(5..=8)
                )
            }) as fn(&mut Self) -> String,
            (|this: &mut Self| {
                format!(
                    "({} rv:{}.0)",
                    this.mac_platform_token(),
                    this.gen_range(2..=7)
                )
            }) as fn(&mut Self) -> String,
        ])(self);
        format!(
            "Mozilla/5.0 {} Gecko/{} Firefox/{}.0",
            platform,
            date,
            self.gen_range(35..=37),
        )
    }

    #[must_use]
    pub fn safari(&mut self) -> String {
        let version = format!(
            "{}.{}.{}",
            self.gen_range(531..=536),
            self.gen_range(1..=51),
            self.gen_range(1..=8),
        );
        let ver = format!("{}.{}", self.gen_range(4..=6), self.gen_range(0..=2));
        let mobile_devices = self.choose(["iPhone; CPU iPhone OS", "iPad; CPU OS"]);
        let platforms = self.choose([
            (|this: &mut Self, version, ver, _mobile_devices|
                format!(
                    "(Windows; U; {}) AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}",
                    this.windows_platform_token(),
                    version,
                    ver,
                    version
                )
            ) as fn(&mut Self, String, String, &str) -> String,
            (|this: &mut Self, version, ver, _mobile_devices|
                format!(
                    "({} rv:{}.0; en-US) AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}",
                    this.mac_platform_token(),
                    this.gen_range(4..=7),
                    version,
                    ver,
                    version
                )
            ) as fn(&mut Self, String, String, &str) -> String,
            (|this: &mut Self, version, _ver, mobile_devices|
                format!(
                    "({} {}_{}_{} like Mac OS X; en-US) AppleWebKit/{} (KHTML, like Gecko) Version/{}.0.5 Mobile/8B{} Safari/6{}",
                    mobile_devices,
                    this.gen_range(7..=9),
                    this.gen_range(0..=3),
                    this.gen_range(1..= 3),
                    version,
                    this.gen_range(3..=5),
                    this.gen_range(111..= 120),
                    version
                )
            ) as fn(&mut Self, String, String, &str) -> String,

        ])(self, version, ver, mobile_devices);

        format!("Mozilla/5.0 {platforms}")
    }

    #[must_use]
    pub fn opera(&mut self) -> String {
        let platform = format!(
            "({}; en-US) Presto/2.{}.{} Version/{}.00",
            self.random_platform(),
            self.gen_range(8..=13),
            self.gen_range(160..=355),
            self.gen_range(10..=13)
        );

        format!(
            "Opera/{}.{} {}",
            self.gen_range(8..=10),
            self.gen_range(10..=99),
            platform
        )
    }

    #[must_use]
    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    pub fn linux_platform_token(&mut self) -> String {
        format!(
            "X11; Linux {}",
            LINUX_PROCESSOR
                .choose(self)
                .expect("LINUX_PROCESSOR should not be empty")
        )
    }

    #[must_use]
    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    pub fn mac_platform_token(&mut self) -> String {
        format!(
            "Macintosh; {} Mac OS X 10_{}_{}",
            MAC_PROCESSOR
                .choose(self)
                .expect("MAC_PROCESSOR should not be empty"),
            self.gen_range(5..=9),
            self.gen_range(0..=10),
        )
    }

    #[must_use]
    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    pub fn windows_platform_token(&mut self) -> String {
        (*WINDOWS_PLATFORM
            .choose(self)
            .expect("WINDOWS_PLATFORM should not be empty"))
        .to_string()
    }

    #[must_use]
    pub fn random_platform(&mut self) -> String {
        self.choose([
            Self::linux_platform_token as fn(&mut Self) -> String,
            Self::mac_platform_token as fn(&mut Self) -> String,
            Self::windows_platform_token as fn(&mut Self) -> String,
        ])(self)
    }
}
