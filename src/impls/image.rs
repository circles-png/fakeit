use rand::{Rng, RngCore};

use crate::Unreal;

/// Generate random image data.
impl<R: RngCore> Unreal<R> {
    #[must_use]
    /// Generate a random image URL with the specified width and height. The seed is a [`u32`] generated from
    /// the internal RNG.
    pub fn image_url(&mut self, width: u32, height: u32) -> String {
        format!(
            "https://picsum.photos/seed/{}/{width}/{height}",
            self.r#gen::<u32>()
        )
    }
}
