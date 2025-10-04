use rkit::{
    draw::{Camera2D, ScreenMode},
    math::Vec2,
    prelude::*,
};

use crate::{
    consts::{GAME_RESOLUTION, UI_RESOLUTION},
    misc::Pos,
};

pub fn camera_plugin(app: &mut App) {
    app.on_setup(init_camera_system)
        .on_pre_update(update_camera_system)
        .on_pre_render(update_camera_system);
}

#[derive(Component, Copy, Clone)]
pub struct GameCam;

#[derive(Component, Copy, Clone)]
pub struct UICam;

#[derive(Component, Copy, Clone, Deref)]
pub struct Cam {
    #[deref]
    pub inner: Camera2D,
    pub mouse_pos: Vec2,
}

impl Cam {
    pub fn new(size: Vec2, resolution: Vec2, is_ui: bool) -> Self {
        let mode = if is_ui {
            ScreenMode::AspectFit(resolution)
        } else {
            ScreenMode::AspectFill(resolution)
        };

        let mut inner = Camera2D::new(size, mode);
        inner.set_pixel_perfect(true);
        inner.update();

        Self {
            inner,
            mouse_pos: Vec2::ZERO,
        }
    }
}

fn init_camera_system(mut cmds: Commands, win: Res<Window>) {
    cmds.spawn((
        UICam,
        Cam::new(win.size(), UI_RESOLUTION, true),
        Pos(UI_RESOLUTION * 0.5),
    ));
    cmds.spawn((
        GameCam,
        Cam::new(win.size(), GAME_RESOLUTION, false),
        Pos(GAME_RESOLUTION * 0.5),
    ));
}

fn update_camera_system(mut cams: Query<(&mut Cam, &Pos)>, mouse: Res<Mouse>, win: Res<Window>) {
    let mouse_pos = mouse.position();
    let win_size = win.size();
    cams.iter_mut().for_each(|(mut cam, pos)| {
        cam.mouse_pos = cam.screen_to_local(mouse_pos);
        cam.set_size(win_size);
        cam.set_position(pos.0);
        cam.update();
    });
}
