use amethyst::utils::application_root_dir;

mod game;
mod paddle;
mod constants;

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir().unwrap();
    let _display_config_path = app_root.join("config/display.ron");
    let _assets_dir = app_root.join("assets/");

    Ok(())
}
