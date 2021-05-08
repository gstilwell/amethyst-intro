extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    prelude::*,
    core::transform::Transform,
    renderer::{
        camera::Camera,
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat},
        Texture},
};

use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};

use crate::paddle::initialize_paddles;
use crate::ball::{Ball, initialize_ball};

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Ball>();

        initialize_ball(world, sprite_sheet_handle.clone());
        initialize_paddles(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}

fn initialize_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        ARENA_WIDTH * 0.5,
        ARENA_HEIGHT * 0.5,
        1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}