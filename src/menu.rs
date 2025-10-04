use rkit::{
    draw::create_draw_2d,
    gfx::{self, Color},
    math::Vec2,
    prelude::*,
};

use crate::AppScreen;

pub fn menu_plugin(app: &mut App) {
    app.on_screen_schedule(AppScreen::Menu, OnRender, draw_system);
}

fn draw_system(win: Res<Window>) {
    let mut draw = create_draw_2d();
    draw.clear(Color::BLACK);

    draw.text("Menu")
        .size(20.0)
        .translate(win.size() * 0.5)
        .origin(Vec2::splat(0.5))
        .h_align_center();

    gfx::render_to_frame(&draw).unwrap();
}
