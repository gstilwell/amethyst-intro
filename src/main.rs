use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub struct Pong;

impl SimpleState for Pong {}

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir().unwrap();
    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets/");
    Ok(())
}
