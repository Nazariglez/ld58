use rkit::{math::Vec2, prelude::*};

use crate::tweens::UITransformScaleTween;

const ANIMATION_TIME: f32 = 0.1;

pub fn anims_plugin(app: &mut App) {
    app.on_update((
        grow_on_focus_system,
        bounce_on_press_system,
        snd_on_enter_system,
        snd_on_press_system,
    ));
}

#[derive(Component, Clone, Copy)]
#[require(UIPointer)]
pub struct GrowOnFocus {
    pub base: Vec2,
    pub grow: Vec2,
}

impl Default for GrowOnFocus {
    fn default() -> Self {
        Self {
            base: Vec2::ONE,
            grow: Vec2::splat(1.2),
        }
    }
}

fn grow_on_focus_system(
    mut cmds: Commands,
    nodes: Query<(Entity, &UITransform, &UIPointer, &GrowOnFocus)>,
) {
    nodes.iter().for_each(|(entity, transform, pointer, fx)| {
        if pointer.just_enter() {
            cmds.entity(entity).tween(
                UITransformScaleTween {
                    from: transform.scale,
                    to: fx.grow,
                },
                ANIMATION_TIME,
            );
        } else if pointer.just_exit() {
            cmds.entity(entity).tween(
                UITransformScaleTween {
                    from: transform.scale,
                    to: fx.base,
                },
                ANIMATION_TIME,
            );
        }
    });
}

#[derive(Component, Deref)]
#[require(UIPointer)]
pub struct BounceOnPress(pub Vec2);

impl Default for BounceOnPress {
    fn default() -> Self {
        Self(Vec2::splat(0.2))
    }
}

fn bounce_on_press_system(
    mut cmds: Commands,
    nodes: Query<(Entity, &UITransform, &UIPointer, &BounceOnPress)>,
) {
    nodes.iter().for_each(|(entity, transform, pointer, fx)| {
        if pointer.just_pressed(MouseButton::Left) {
            cmds.entity(entity)
                .tween(
                    UITransformScaleTween {
                        from: transform.scale,
                        to: transform.scale + fx.0,
                    },
                    ANIMATION_TIME,
                )
                .yoyo(true);
        }
    });
}

#[derive(Component, Deref)]
#[require(UIPointer)]
pub struct SoundOnEnter(pub Sound);

fn snd_on_enter_system(nodes: Query<(&UIPointer, &SoundOnEnter)>, mut audio: ResMut<Audio>) {
    nodes.iter().for_each(|(pointer, snd)| {
        if pointer.just_enter() {
            audio.play(&snd.0);
        }
    });
}

#[derive(Component, Deref)]
#[require(UIPointer)]
pub struct SoundOnPress(pub Sound);

fn snd_on_press_system(nodes: Query<(&UIPointer, &SoundOnPress)>, mut audio: ResMut<Audio>) {
    nodes.iter().for_each(|(pointer, snd)| {
        if pointer.just_clicked(MouseButton::Left) {
            audio.play(&snd.0);
        }
    });
}
