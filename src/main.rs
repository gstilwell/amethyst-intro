use amethyst::utils::application_root_dir;

mod game;
use crate::game::Game;

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir().unwrap();
    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets/");

    Ok(())
}
