use bevy::asset::AssetServer;
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
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        // Runs before any of the startup systems.
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

// Just hold the copy of the ascii handle.
struct AsciiSheet(Handle<TextureAtlas>);

// Graphics
// --------
// Spritesheets and other assets.

// Assets
// ------
// They *need* to be placed at "./assets" folder in order to Bevy's assets
// manager find your files.

// Atlas
// -----
// An atlas is a texture that contains multiple smaller textures. Its a map that
// points to the location of each texture in a spritesheet.

fn load_ascii(
    mut commands: Commands,
    // Its added by 'DefaultPlugins'.
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("ascii.png");

    // We're using padding to prevent the bleeding of pixels in the tiles.
    let atlas =
        TextureAtlas::from_grid_with_padding(image, Vec2::splat(9.0), 16, 16, Vec2::splat(2.0));

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);

    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(1.0));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"));
}
