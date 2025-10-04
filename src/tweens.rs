use rkit::{math::Vec2, prelude::*};

pub fn tweens_plugin(app: &mut App) {
    app.add_plugin(TweenPlugin::<UITransform, UITransformScaleTween>::default());
}

pub struct UITransformScaleTween {
    pub from: Vec2,
    pub to: Vec2,
}

impl TweenableComponent<UITransform> for UITransformScaleTween {
    fn tick(&mut self, target: &mut UITransform, progress: f32) {
        target.scale = self.from.lerp(self.to, progress);
    }
}
