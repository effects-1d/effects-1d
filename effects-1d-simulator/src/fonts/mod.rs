use bevy::{asset::load_internal_binary_asset, prelude::*};

const ROBOTO_FONT_HANDLE: Handle<Font> = Handle::weak_from_u128(3730976889387404451);

pub fn roboto() -> Handle<Font> {
    ROBOTO_FONT_HANDLE
}

pub struct RobotoFontPlugin;
impl Plugin for RobotoFontPlugin {
    fn build(&self, app: &mut App) {
        load_internal_binary_asset!(
            app,
            ROBOTO_FONT_HANDLE,
            "Roboto-Regular.ttf",
            |data: &[u8], _path: String| { Font::try_from_bytes(data.to_vec()).unwrap() }
        );
    }
}
