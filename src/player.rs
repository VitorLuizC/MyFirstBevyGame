use bevy::prelude::*;

use crate::{AsciiSheet, TILE_SIZE};

#[derive(Component)]
pub struct Player {
    speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system(move_player);
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut face_sprite = TextureAtlasSprite::new(1);

    face_sprite.color = Color::rgb(0.3, 0.3, 0.9);
    face_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let face = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: face_sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player { speed: 3.0 })
        .id();

    let mut background_sprite = TextureAtlasSprite::new(0);

    background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
    background_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let background = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: background_sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Background"))
        .id();

    commands.entity(face).push_children(&[background]);
}

fn move_player(
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (player, mut transform) = query.single_mut();

    let distance = 0.5 * player.speed * time.delta_seconds();

    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += distance;
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += distance;
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= distance;
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= distance;
    }
}
