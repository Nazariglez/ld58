use std::f32::consts::PI;

use rkit::{
    draw::{BaseCam2D, create_draw_2d},
    gfx::{self, Color},
    math::{Vec2, vec2},
    prelude::*,
};

use crate::{
    AppScreen,
    cam::{Cam, UICam},
    consts::palette,
    postfx::rtf,
};

pub fn load_plugin(app: &mut App) {
    app.on_screen_schedule(AppScreen::Load, OnRender, draw_system)
        .on_screen_schedule(AppScreen::Load, OnUpdate, update_system);
}

fn update_system(mut cmds: Commands, mouse: Res<Mouse>) {
    let can_start = mouse.just_pressed(MouseButton::Left);
    if can_start {
        cmds.queue(ChangeScreen(AppScreen::Menu));
    }
}

fn draw_system(cam: Single<&Cam, With<UICam>>, win: Res<Window>, time: Res<Time>) {
    let mut draw = create_draw_2d();
    draw.clear(palette::BLACK);
    draw.set_camera(&cam.inner);

    let elapsed = time.elapsed_f32();

    let alpha_min = 0.3;
    let alpha_max = 1.0;
    let fade_time = 2.0;
    let move_time = 3.0;

    let fade = elapsed * (2.0 * PI) / fade_time;
    let movement = elapsed * (2.0 * PI) / move_time;

    let alpha = alpha_min + (alpha_max - alpha_min) * (0.5 + 0.5 * fade.sin());
    let offset_y = 10.0 * movement.sin();

    let pos = cam.resolution() * 0.5 + vec2(0.0, offset_y);
    draw.text("Click to start")
        .color(palette::WHITE)
        .size(40.0)
        .alpha(alpha)
        .translate(pos)
        .origin(Vec2::splat(0.5))
        .h_align_center();

    rtf(&draw).unwrap();
}
