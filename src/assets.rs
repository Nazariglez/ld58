use rkit::{audio::create_sound, prelude::*};

pub fn assets_plugin(app: &mut App) {
    app.on_setup(setup_assets_system);
}

fn setup_assets_system(mut cmds: Commands) {
    let snd_press_btn =
        create_sound(include_bytes!("../assets/click1.ogg")).or_panic("Creating sound");
    let snd_enter_btn =
        create_sound(include_bytes!("../assets/click5.ogg")).or_panic("Creating sound");

    cmds.insert_resource(Assets {
        snd_enter_btn,
        snd_press_btn,
    });
}

#[derive(Resource)]
pub struct Assets {
    pub snd_enter_btn: Sound,
    pub snd_press_btn: Sound,
}
