mod consts;
mod game;
mod load;
mod menu;
mod postfx;

use rkit::prelude::*;

use crate::{game::game_plugin, load::load_plugin, menu::menu_plugin};

pub fn main() -> Result<(), String> {
    App::new()
        // framework
        .add_plugin(MainPlugins::default())
        .add_plugin(AudioPlugin)
        .add_plugin(window_plugin())
        .add_plugin(logging_plugin())
        // game
        .add_screen(AppScreen::Load)
        .add_plugin(load_plugin)
        .add_plugin(menu_plugin)
        .add_plugin(game_plugin)
        .run()
}

fn window_plugin() -> WindowConfigPlugin {
    let size = consts::GAME_RESOLUTION.as_uvec2();
    let plugin = WindowConfigPlugin::default()
        .title("LD58")
        .max_fps(120)
        .vsync(true)
        .pixelated(true)
        .size(size.x, size.y);

    #[cfg(target_arch = "wasm32")]
    let plugin = plugin.maximized(true);

    plugin
}

fn logging_plugin() -> LogPlugin {
    let needs_trace = option_env!("TRACE").is_some_and(|v| v == "on" || v == "1");
    let is_prod = !cfg!(debug_assertions);
    if needs_trace {
        LogPlugin::trace()
    } else if is_prod {
        LogPlugin::info()
    } else {
        LogPlugin::debug()
    }
}

#[derive(Screen, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum AppScreen {
    Load,
    Menu,
    Game,
}
