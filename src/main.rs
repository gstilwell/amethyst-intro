mod game;
mod paddle;
mod constants;

use amethyst::{
    core::transform::TransformBundle,
    utils::application_root_dir,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
};

use crate::game::Game;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default());

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(rendering_bundle)?;

    let mut game = Application::new(assets_dir, Game, game_data)
        .expect("Failed to initialize");
    
    game.run();

    Ok(())
}