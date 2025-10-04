mod anims;
mod assets;
mod cam;
mod consts;
mod game;
mod load;
mod menu;
mod misc;
mod postfx;
mod tweens;

use rkit::prelude::*;

use crate::{
    anims::anims_plugin, assets::assets_plugin, cam::camera_plugin, game::game_plugin,
    load::load_plugin, menu::menu_plugin, misc::misc_plugin, postfx::postfx_plugin,
    tweens::tweens_plugin,
};

pub fn main() -> Result<(), String> {
    let init_screen = if cfg!(debug_assertions) {
        AppScreen::Load
    } else {
        AppScreen::Load
    };
    App::new()
        // framework
        .add_plugin(MainPlugins::default())
        .add_plugin(AudioPlugin)
        .add_plugin(window_plugin())
        .add_plugin(logging_plugin())
        // game
        .add_screen(init_screen)
        .add_plugin(camera_plugin)
        .add_plugin(assets_plugin)
        .add_plugin(misc_plugin)
        .add_plugin(load_plugin)
        .add_plugin(menu_plugin)
        .add_plugin(game_plugin)
        .add_plugin(postfx_plugin)
        .add_plugin(tweens_plugin)
        .add_plugin(anims_plugin)
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
