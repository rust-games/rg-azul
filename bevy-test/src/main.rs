//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 700.0,
                        height: 700.0,
                        ..default()
                    },
                    ..default()
                })
                .set(AssetPlugin {
                    //asset_folder: "".to_string(),
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Direction::Up,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => {
                transform.translation.x += 150. * time.delta_seconds();
                transform.translation.y += 150. * time.delta_seconds();
            }
            Direction::Down => {
                transform.translation.x -= 150. * time.delta_seconds();
                transform.translation.y -= 150. * time.delta_seconds();
            }
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}
