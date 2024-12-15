use rand::RngCore;

use crate::Unreal;

impl<R: RngCore> Unreal<R> {
    #[must_use]
    pub fn image_url(width: u32, height: u32) -> String {
        format!("https://picsum.photos/{width}/{height}")
    }
}
