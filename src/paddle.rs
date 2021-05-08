use amethyst::{
    ecs::{World, Component, DenseVecStorage},
    core::transform::Transform,
    assets::Handle,
    renderer::sprite::{SpriteSheet, SpriteRender},
    prelude::*,
};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH,
                    PADDLE_HEIGHT, PADDLE_WIDTH};

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            height: PADDLE_HEIGHT,
            width: PADDLE_WIDTH,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Create a left plank entity.
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .with(sprite_render)
        .build();
}
