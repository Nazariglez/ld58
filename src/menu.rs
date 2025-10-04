use rkit::{
    draw::{BaseCam2D, HAlign, RenderSprite, Sprite, create_draw_2d, create_render_sprite},
    gfx::{self, TextureFilter},
    math::Vec2,
    postfx::PostProcess,
    prelude::*,
};
use strum_macros::EnumIter;

use crate::{
    AppScreen,
    anims::{BounceOnPress, GrowOnFocus, SoundOnEnter, SoundOnPress},
    assets::Assets,
    cam::{Cam, GameCam, UICam},
    consts::{GAME_RESOLUTION, palette},
    misc::{UIBtn, UIBtnClick},
    postfx::{RenderData, rtf},
};

pub fn menu_plugin(app: &mut App) {
    app.add_plugin(UILayoutPlugin::<UIMenuLayout>::default())
        .on_schedule(OnEnter(AppScreen::Menu), setup)
        .on_schedule(OnExit(AppScreen::Menu), cleanup)
        .on_screen_schedule(AppScreen::Menu, OnRender, draw_system)
        .on_screen_schedule(AppScreen::Menu, OnUpdate, update_system)
        .on_event(on_btn_click_system);
}

#[derive(Component, Clone, Copy)]
pub struct UIMenuLayout;

#[derive(Component, Clone, Copy)]
pub struct InMenuScreen;

#[derive(Component, Clone, Copy, EnumIter)]
pub enum MenuBtn {
    Start,
    Credits,
}

impl MenuBtn {
    pub fn text(&self) -> &str {
        match self {
            MenuBtn::Start => "Start Game",
            MenuBtn::Credits => "Credits",
        }
    }
}

#[derive(Resource)]
pub struct MenuState;

fn setup(mut cmds: Commands, assets: Res<Assets>) {
    cmds.insert_resource(MenuState);

    // -- ui
    let menu_container = cmds
        .spawn_ui_node(
            UIMenuLayout,
            (
                UIContainer {
                    // bg_color: Some(palette::RED),
                    ..Default::default()
                },
                UIStyle::default()
                    .flex_col()
                    .size_full()
                    .gap_y(32.0)
                    .align_items_center()
                    .justify_content_center(),
            ),
        )
        .entity_id();

    [MenuBtn::Start, MenuBtn::Credits]
        .into_iter()
        .for_each(|btn| {
            let btn = cmds
                .spawn_ui_node(
                    UIMenuLayout,
                    (
                        UIBtn,
                        btn,
                        UIText {
                            text: btn.text().to_string(),
                            color: palette::WHITE,
                            size: 32.0,
                            h_align: HAlign::Center,
                            ..Default::default()
                        },
                        GrowOnFocus::default(),
                        BounceOnPress::default(),
                        SoundOnEnter(assets.snd_enter_btn.clone()),
                        SoundOnPress(assets.snd_press_btn.clone()),
                    ),
                )
                .entity_id();

            cmds.add_ui_child(UIMenuLayout, menu_container, btn);
        });
}

fn cleanup(
    mut cmds: Commands,
    entities: Query<Entity, With<InMenuScreen>>,
    ui_nodes: Query<Entity, With<UIMenuLayout>>,
) {
    entities.iter().for_each(|entity| {
        cmds.entity(entity).try_despawn();
    });

    ui_nodes.iter().for_each(|entity| {
        cmds.despawn_ui_node(UIMenuLayout, entity);
    });
}

fn update_system(cam: Single<&Cam, With<UICam>>, mut layout: ResMut<UILayout<UIMenuLayout>>) {
    layout.set_camera(&cam.inner);
}

fn on_btn_click_system(evt: On<UIBtnClick>, mut cmds: Commands, btns: Query<&MenuBtn>) {
    let entity = evt.0;
    let Ok(btn) = btns.get(entity) else {
        return;
    };

    match btn {
        MenuBtn::Start => cmds.queue(ChangeScreen(AppScreen::Game)),
        _ => {}
    }
}

fn draw_system(world: &mut World) {
    world
        .run_system_cached(draw_world_system)
        .or_panic("Drawing world");
    world
        .run_system_cached(draw_menu_system)
        .or_panic("Drawing menu");
}

fn draw_menu_system(world: &mut World) {
    {
        let ui_cam = world
            .query_filtered::<&Cam, With<UICam>>()
            .single(world)
            .or_panic("Getting UI cam");

        let mut draw = create_draw_2d();
        draw.clear(palette::BLACK.with_alpha(0.5));
        draw.set_camera(&ui_cam.inner);

        draw_ui_layout::<UIMenuLayout>(&mut draw, world);

        let render_data = world.resource::<RenderData>();
        let pfx = PostProcess {
            effects: &[&render_data.shadow_fx],
            render: &draw,
            nearest_sampler: true,
            clear_target: true,
        };

        gfx::render_to_texture(&render_data.menu_rt.render_texture, &pfx)
            .or_panic("Rendering menu to texture");
    }

    let ui_cam = world
        .query_filtered::<&Cam, With<UICam>>()
        .single(world)
        .or_panic("Getting UI cam");

    let render_data = world.resource::<RenderData>();
    draw_sprite_to_screen(&ui_cam, &render_data.menu_rt.sprite);
}

fn draw_world_system(game_cam: Single<&Cam, With<GameCam>>, render_data: Res<RenderData>) {
    let mut draw = create_draw_2d();
    draw.clear(palette::DARK_GREEN);
    draw.set_camera(&game_cam.inner);
    draw.text("This is the world")
        .size(10.0)
        .translate(GAME_RESOLUTION * 0.5 - Vec2::new(0.0, 100.0))
        .origin(Vec2::splat(0.5))
        .h_align_center();

    let pfx = PostProcess {
        effects: &[&render_data.blur_fx],
        render: &draw,
        nearest_sampler: true,
        clear_target: true,
    };

    gfx::render_to_texture(&render_data.world_rt.render_texture, &pfx)
        .or_panic("Rendering world to texture");

    draw_sprite_to_screen(&game_cam, &render_data.world_rt.sprite);
}

fn draw_sprite_to_screen(cam: &Cam, sprite: &Sprite) {
    let mut draw = create_draw_2d();
    draw.set_camera(&cam.inner);
    draw.image(sprite).translate(cam.bounds().min());
    rtf(&draw).unwrap();
}
