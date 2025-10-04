use rkit::{
    draw::{RenderSprite, create_render_sprite},
    gfx::{AsRenderer, Color, TextureFilter},
    math::{UVec2, Vec2},
    postfx::{self, BlurFx, BlurParams, CrtFx, CrtParams, PostFx, ShadowFx, ShadowParams},
    prelude::*,
};

use crate::cam::{Cam, GameCam, UICam};

pub fn postfx_plugin(app: &mut App) {
    app.on_setup(init_postfx_system)
        .on_pre_render(update_postfx_system)
        .on_post_render(render_postfx_system);
}

// Render to PostFX Frame
#[inline]
pub fn rtf<R: AsRenderer>(renderer: &R) -> Result<(), String> {
    postfx::render_to_pfx_frame(renderer)
}

fn init_postfx_system(mut cmds: Commands) {
    let pfx = RenderData::new().or_panic("Initiating PostFxRender");
    cmds.insert_resource(pfx);
}

fn update_postfx_system(
    game_cam: Single<&Cam, With<GameCam>>,
    ui_cam: Single<&Cam, With<UICam>>,
    mut pfx: ResMut<RenderData>,
    win: Res<Window>,
) {
    pfx.crt_fx.params.scanline_count = ((300.0 * win.height()) / 600.0).min(300.0);
    pfx.update(game_cam.size_visible(), ui_cam.size_visible())
        .or_panic("Updating PostFX");
}

fn render_postfx_system(pfx: ResMut<RenderData>) {
    postfx::present_pfx_frame(&pfx.screen_effects(), true, true).or_panic("Presenting PFX Frame");
}

#[derive(Resource)]
pub struct RenderData {
    pub crt_fx: CrtFx,
    pub blur_fx: BlurFx,
    pub shadow_fx: ShadowFx,

    pub world_rt: RenderSprite,
    pub menu_rt: RenderSprite,
}

impl RenderData {
    fn new() -> Result<Self, String> {
        let crt_fx = CrtFx::new(CrtParams {
            scanline_count: 300.0,
            curvature_amount: 3.2,
            chromatic_aberration_amount: 0.0005,
            scanline_intensity: 0.40,
            roll_line_offset: 1.0,
            roll_speed: 0.04,
            roll_height: 310.0,
            scale_factor: 0.999,
            vignette_width: 6.0,
        })?;
        let blur_fx = BlurFx::new(BlurParams {
            strength: 2.0,
            quality: 8.0,
            ..Default::default()
        })?;
        let shadow_fx = ShadowFx::new(ShadowParams {
            color: Color::BLACK,
            offset: Vec2::splat(4.0),
            ..Default::default()
        })?;

        let (world_rt, menu_rt) = create_render_sprites(UVec2::ONE, UVec2::ONE)?;

        Ok(Self {
            crt_fx,
            blur_fx,
            shadow_fx,
            world_rt,
            menu_rt,
        })
    }

    pub fn update(&mut self, game_size: Vec2, ui_size: Vec2) -> Result<(), String> {
        self.crt_fx.update()?;
        self.blur_fx.update()?;
        self.shadow_fx.update()?;

        let is_game_size_changed = self.world_rt.render_texture.size() != game_size;
        let is_ui_size_changed = self.menu_rt.render_texture.size() != ui_size;
        let is_size_changed = is_game_size_changed || is_ui_size_changed;
        if is_size_changed {
            let (world_rt, menu_rt) =
                create_render_sprites(ui_size.as_uvec2(), game_size.as_uvec2())?;
            self.world_rt = world_rt;
            self.menu_rt = menu_rt;
        }

        Ok(())
    }

    pub fn screen_effects(&self) -> [&dyn PostFx; 1] {
        [&self.crt_fx]
    }
}

fn create_render_sprites(
    ui_size: UVec2,
    world_size: UVec2,
) -> Result<(RenderSprite, RenderSprite), String> {
    let world_rt = create_render_sprite()
        .with_size(world_size.x, world_size.y)
        .with_filter(TextureFilter::Nearest)
        .with_label("World RT")
        .build()
        .map_err(|_| "Creating World RenderSprite".to_string())?;

    let menu_rt = create_render_sprite()
        .with_size(ui_size.x, ui_size.y)
        .with_filter(TextureFilter::Nearest)
        .with_label("Menu RT")
        .build()
        .map_err(|_| "Creating Menu RenderSprite".to_string())?;

    Ok((world_rt, menu_rt))
}
