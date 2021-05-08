extern crate amethyst;

use amethyst::{
    //assets::{AssetStorage, Loader, Handle},
    prelude::*,
    core::transform::Transform,
    renderer::Camera,
    //renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};

use crate::paddle::{Paddle, initialize_paddles};

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Paddle>();
        initialize_paddles(world);
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
