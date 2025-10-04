use rkit::{math::Vec2, prelude::*};

pub fn misc_plugin(app: &mut App) {
    app.on_update(trigger_ui_btn_click_system);
}

#[derive(Component, Copy, Clone, Deref, Default)]
pub struct Pos(pub Vec2);

// -- ui
#[derive(Component, Copy, Clone)]
pub struct UIBtn;

#[derive(Component, Copy, Clone, EntityEvent, Debug)]
pub struct UIBtnClick(pub Entity);

fn trigger_ui_btn_click_system(
    mut cmds: Commands,
    nodes: Query<(Entity, &UIPointer), With<UIBtn>>,
) {
    nodes.iter().for_each(|(entity, pointer)| {
        if pointer.just_pressed(MouseButton::Left) {
            cmds.trigger(UIBtnClick(entity));
        }
    });
}
