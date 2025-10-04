use rkit::math::Vec2;

pub const GAME_RESOLUTION: Vec2 = Vec2::new(640.0, 360.0);
pub const UI_RESOLUTION: Vec2 = Vec2::new(1280.0, 720.0);

pub mod palette {
    use rkit::gfx::Color;

    // Color Palette https://lospec.com/palette-list/sweetie-16
    pub const BLACK: Color = Color::rgba_u8(26, 28, 44, 255); // #1a1c2c
    pub const PURPLE: Color = Color::rgba_u8(93, 39, 93, 255); // #5d275d
    pub const RED: Color = Color::rgba_u8(177, 62, 83, 255); // #b13e53
    pub const ORANGE: Color = Color::rgba_u8(239, 125, 87, 255); // #ef7d57
    pub const YELLOW: Color = Color::rgba_u8(255, 205, 117, 255); // #ffcd75
    pub const LIGHT_GREEN: Color = Color::rgba_u8(167, 240, 112, 255); // #a7f070
    pub const GREEN: Color = Color::rgba_u8(56, 183, 100, 255); // #38b764
    pub const TEAL: Color = Color::rgba_u8(37, 113, 121, 255); // #257179
    pub const NAVY: Color = Color::rgba_u8(41, 54, 111, 255); // #29366f
    pub const BLUE: Color = Color::rgba_u8(59, 93, 201, 255); // #3b5dc9
    pub const LIGHT_BLUE: Color = Color::rgba_u8(65, 166, 246, 255); // #41a6f6
    pub const CYAN: Color = Color::rgba_u8(115, 239, 247, 255); // #73eff7
    pub const WHITE: Color = Color::rgba_u8(244, 244, 244, 255); // #f4f4f4
    pub const LIGHT_GRAY: Color = Color::rgba_u8(148, 176, 194, 255); // #94b0c2
    pub const GRAY: Color = Color::rgba_u8(86, 108, 134, 255); // #566c86
    pub const DARK_GRAY: Color = Color::rgba_u8(51, 60, 87, 255); // #333c57

    pub const DARK_GREEN: Color = Color::rgba_u8(28, 92, 50, 255); // #1c5c32, darker than GREEN
}
