use bevy_egui::EguiPlugin;
use consts::{HEIGHT, TILE_HEIGHT, TILE_SIZE, TILE_WIDTH, WIDTH};
mod board;
mod consts;
mod controls;
mod physics;
mod tile;
use crate::physics::PhysicsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board::BoardPlugin;
use controls::ControlsPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Particles".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(Time::<Fixed>::from_hz(4.))
        .add_plugins((
            BoardPlugin,
            // WorldInspectorPlugin::new(),
            EguiPlugin,
            ControlsPlugin,
            FrameTimeDiagnosticsPlugin,
            PhysicsPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        transform: Transform::from_xyz(
            TILE_HEIGHT as f32 * TILE_SIZE / 2.,
            TILE_WIDTH as f32 * TILE_SIZE / 2.,
            0.,
        )
        .with_scale(Vec3::new(2., 2., 1.)),
        ..Default::default()
    };
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_height: HEIGHT,
        min_width: WIDTH,
    };
    commands.spawn(camera);
}
