use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PresentMode;

pub const CLEAR: Color = Color::rgb(0.0, 0.0, 0.0);

pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let height: f32 = 800.0;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "MyFirstBevyGame".to_string(),

            // 'vsync' was replaced by 'present_mode' in Bevy 0.7.0, and to have
            // same behavior its value must be 'PresentMode::Fifo'.
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .run();
}

// Command
// ------
// Commands let us create entities and modify their components. They are
// only executed at the end of the frame, so we just see the changes in the
// next frame.

fn spawn_camera(mut commands: Commands) {
    // Bundle
    // ------
    // Bundles are group of components packed for easy use.

    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
