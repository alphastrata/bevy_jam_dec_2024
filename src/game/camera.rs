use bevy::{core::Zeroable, log, prelude::*, window::PrimaryWindow};

use crate::AppState;

use super::keybinds::FloraCommand;

/// Play with this to modify the multiplier for camera pan movement
const PAN_SPEED: f32 = 8.0;

const CAMERA_BOUNDS_MIN: Vec2 = Vec2::new(-2000.0, -2000.0);
const CAMERA_BOUNDS_MAX: Vec2 = Vec2::new(2000.0, 2000.0);

const FRICTION: Vec2 = Vec2::splat(0.85);
const ACCELERATION: Vec2 = Vec2::splat(100.0);
const VELOCITY_MAX: Vec2 = Vec2::splat(1000.0);

/// Component that adds our gameplay camera controls
#[derive(Component)]
pub struct GameCamera {
    zoom_current: f32,
    zoom_target: f32,
    velocity: Vec2,
}

impl Default for GameCamera {
    fn default() -> Self {
        GameCamera {
            zoom_current: 1.0,
            zoom_target: 1.0,
            velocity: Vec2::ZERO,
        }
    }
}

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_camera).run_if(in_state(AppState::Playing)));
    }
}

fn move_camera(
    input: Res<Input<FloraCommand>>,
    mut query: Query<(&mut GameCamera, &mut Transform, &OrthographicProjection)>,
    time: Res<Time>,
) {
    let (mut state, mut transform, _proj) = query.single_mut();

    let mut accel = Vec2::ZERO;

    if input.pressed(FloraCommand::Left) {
        accel -= Vec2::X;
    }
    if input.pressed(FloraCommand::Right) {
        accel += Vec2::X;
    }
    if input.pressed(FloraCommand::Up) {
        accel += Vec2::Y;
    }
    if input.pressed(FloraCommand::Down) {
        accel -= Vec2::Y;
    }

    accel = accel.normalize_or_zero();
    state.velocity += accel * Vec2::splat(time.delta_seconds()) * ACCELERATION;

    transform.translation += Vec3::from((state.velocity, 0.0));
    state.velocity *= FRICTION;
}

/*fn pan_camera(
    keys: Res<Input<KeyCode>>,
    mouse_btns: Res<Input<MouseButton>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&GameCamera, &mut Transform, &OrthographicProjection)>,
    mut last_pos: Local<Option<Vec2>>,
) {
    let window = primary_window.single();
    let (_game_cam, mut transform, _proj) = query.single_mut();

    let mut direction_vecs = vec![];
    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
        direction_vecs.push(Vec3::NEG_Y)
    }
    if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
        direction_vecs.push(Vec3::Y)
    }
    if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
        direction_vecs.push(Vec3::X)
    }
    if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
        direction_vecs.push(Vec3::NEG_X)
    }

    let camera_move_vector = direction_vecs
        .into_iter()
        .fold(Vec3::ZERO, |avg, vec| avg + vec)
        .try_normalize();

    if let Some(direction) = camera_move_vector {
        log::info!("Camera move {direction}");
        debug_assert!(direction.z == 0.0);

        transform.translation += PAN_SPEED * direction;
        // WASD takes precedence over mouse dragging so early exit here
        return;
    }

    let current_pos = match window.cursor_position() {
        Some(p) => Vec2::new(p.x, -p.y), // Y Positive for mouse is Y Negative for world-space
        None => return,                  // mouse is outside the window
    };
    if mouse_btns.pressed(MouseButton::Left) {
        let mouse_delta = current_pos - last_pos.unwrap_or(current_pos);
        transform.translation -= Vec3::new(mouse_delta.x, mouse_delta.y, 0.0);
    }
    *last_pos = Some(current_pos);
}
*/
