#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn zero() -> Vec2 {
        return Vec2 { x: 0.0, y: 0.0 };
    }

    fn add(self: Vec2, v: Vec2) -> Vec2 {
        return Vec2 {
            x: self.x + v.x,
            y: self.y + v.y,
        };
    }

    pub fn sub(self: Vec2, v: Vec2) -> Vec2 {
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

impl Rect {
    pub fn aabb_check(self: &Rect, v: &Vec2) -> bool {
        return (v.x >= self.x && v.x <= self.x + self.width)
            && (v.y >= self.y && v.y <= self.y + self.height);
    }
}
