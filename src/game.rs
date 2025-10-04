use rkit::{draw::create_draw_2d, gfx::Color, math::Vec2, prelude::*};

use crate::{
    AppScreen,
    cam::{Cam, UICam},
    consts::UI_RESOLUTION,
    postfx::rtf,
};

pub fn game_plugin(app: &mut App) {
    app.on_screen_schedule(AppScreen::Game, OnRender, draw_system);
}

fn draw_system(ui_cam: Single<&Cam, With<UICam>>) {
    let mut draw = create_draw_2d();
    draw.clear(Color::BLACK);
    draw.set_camera(&ui_cam.inner);

    draw.text("Game")
        .size(20.0)
        .translate(UI_RESOLUTION * 0.5)
        .origin(Vec2::splat(0.5))
        .h_align_center();

    rtf(&draw).unwrap();
}
