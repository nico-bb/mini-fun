pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    fn add(self: Vec2, v: Vec2) -> Vec2 {
        return Vec2 {
            x: self.x + v.x,
            y: self.y + v.y,
        };
    }

    fn sub(self: Vec2, v: Vec2) -> Vec2 {
        return Vec2 {
            x: self.x - v.x,
            y: self.y - v.y,
        };
    }

    fn scale(self: Vec2, s: f32) -> Vec2 {
        return Vec2 {
            x: self.x * s,
            y: self.y * s,
        };
    }
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
