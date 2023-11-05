macro_rules! load_sprite {
    ($name:literal, $size:literal, $asset_server:expr) => {
        load_sprite!($name, ($size, $size), $asset_server)
    };
    ($name:literal, ($width:literal, $height:literal), $asset_server:expr) => {{
        let mut sprite = bevy::sprite::SpriteBundle {
            texture: $asset_server.load($name),
            ..default()
        };
        sprite.transform.scale = Vec3::new($width, $height, 0.0);
        sprite
    }};
}
pub(crate) use load_sprite;
