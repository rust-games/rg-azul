//! module of systems.

use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::log::*;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::constant::*;
use crate::ecs::component::{PlayerInput, TileColor, TileRegion};

fn pool_setup_system(mut commands: Commands) {
    for _ in 0..NUMBER_OF_TILES_PER_COLOR {
        commands.spawn((TileColor::Blue, TileRegion::Pool));
        commands.spawn((TileColor::Yellow, TileRegion::Pool));
        commands.spawn((TileColor::Red, TileRegion::Pool));
        commands.spawn((TileColor::Black, TileRegion::Pool));
        commands.spawn((TileColor::White, TileRegion::Pool));
    }
}

fn input_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut player_input: ResMut<PlayerInput>,
) {
    if mouse_input.just_release(MouseButton::Left) {
        player_input.mouse_click = mouse.delta
    }

    for event in mouse_button_input_events.iter() {
        match event {
            MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Released,
            } => {
                info!("{:?}", event);
            }
            _ => {}
        }
    }

    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
    }
}

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
    }
}
