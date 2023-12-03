use bevy::{prelude::*, window::PrimaryWindow};

use crate::{camera::GameCamera, towers::spawn_fire_tower};

/// Spawn towers when clicked
pub struct TowerPlacementPlugin;
impl Plugin for TowerPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_at_click_pos);
    }
}

fn spawn_at_click_pos(
    commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mouse_btns: Res<Input<MouseButton>>,
) {
    if mouse_btns.just_pressed(MouseButton::Right) {
        let window = q_window.single();
        let (camera, camera_transform) = q_camera.single();

        if let Some(worldspace_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            spawn_fire_tower(commands, worldspace_pos);
        }
    }
}
