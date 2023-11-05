use bevy::prelude::*;

use crate::macros::load_sprite;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, (player_controller, rotate_player_to_mouse));
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    velocity: Vec2,
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = load_sprite!("ship_sidesB.png", 0.3, asset_server);

    commands
        .spawn((
            Player {
                speed: 256.0,
                velocity: Vec2::default(),
            },
            InheritedVisibility::VISIBLE,
            TransformBundle::default(),
        ))
        .with_children(|parent| {
            parent.spawn(sprite);
        });
}

fn rotate_player_to_mouse(q_window: Query<&Window>, q_camera: Query<(&Camera, &GlobalTransform)>) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    let _mouse_pos = if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        world_position
    } else {
        Vec2::ZERO
    };
}

fn player_controller(
    mut query: Query<(&mut Player, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = query.single_mut();
    let mut direction = Vec2::ZERO;

    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::W | KeyCode::Up => direction.y += 1.0,
            KeyCode::S | KeyCode::Down => direction.y -= 1.0,
            KeyCode::A | KeyCode::Left => direction.x -= 1.0,
            KeyCode::D | KeyCode::Right => direction.x += 1.0,
            _ => {}
        }
    }

    if direction.length() > 1.0 {
        direction = direction.normalize_or_zero();
    }

    let friction = 0.99;
    if direction == Vec2::ZERO {
        player.velocity *= friction;

        if 0.01 > player.velocity.x && player.velocity.x > -0.01 {
            player.velocity.x = 0.0;
        }
        if 0.01 > player.velocity.y && player.velocity.y > -0.01 {
            player.velocity.y = 0.0;
        }
    } else if direction != Vec2::ZERO {
        player.velocity = direction * player.speed;
    }

    // let max_speed = player.speed;
    // player.velocity = player.velocity.clamp_length_max(max_speed);

    dbg!(&player.velocity, direction);

    let dt = time.delta_seconds();
    transform.translation.x += player.velocity.x * dt;
    transform.translation.y += player.velocity.y * dt;
}
