use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, player_controller);
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut sprite = SpriteBundle {
        texture: asset_server.load("ship_sidesB.png"),
        ..default()
    };
    sprite.transform.scale = Vec3::new(0.3, 0.3, 0.0);

    commands
        .spawn((
            Player { speed: 256.0 },
            InheritedVisibility::VISIBLE,
            TransformBundle::default(),
        ))
        .with_children(|parent| {
            parent.spawn(sprite);
        });
}

fn player_controller(
    mut query: Query<(&Player, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();
    let mut movement_vector = Vec3::ZERO;

    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        movement_vector.y = 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        movement_vector.y = -1.0;
    }
    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        movement_vector.x = -1.0;
    }
    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        movement_vector.x = 1.0;
    }

    transform.translation +=
        movement_vector.normalize_or_zero() * player.speed * time.delta_seconds();
}
