use bevy::prelude::*;
use bevy::window::PresentMode;

pub const CLEAR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "MyFirstBevyGame".to_string(),

            // 'vsync' was replaced by 'present_mode' in Bevy 0.7.0, and to have
            // same behavior its value must be 'PresentMode::Fifo'.
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
