use bevy::{asset::load_internal_binary_asset, prelude::*, reflect::TypeUuid};

const ROBOTO_FONT_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Font::TYPE_UUID, 3730976889387404451);

pub fn roboto() -> Handle<Font> {
    ROBOTO_FONT_HANDLE.typed()
}

pub struct RobotoFontPlugin;
impl Plugin for RobotoFontPlugin {
    fn build(&self, app: &mut App) {
        load_internal_binary_asset!(
            app,
            ROBOTO_FONT_HANDLE,
            "Roboto-Regular.ttf",
            |data: &[u8]| { Font::try_from_bytes(data.to_vec()).unwrap() }
        );
    }
}
