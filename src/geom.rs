#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn dst_sqr(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn is_in_radius_of(&self, other: &Point, r: i64) -> bool {
        self.dst_sqr(other) <= r * r
    }

    pub fn in_radius_with_direction(&self, other: &Point, r: i64) -> Point {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let t = r as f32 / ((dx * dx + dy * dy) as f32).sqrt();
        if t <= 1f32 {
            let x = (self.x as f32) * (1f32 - t) + (other.x as f32) * t;
            let y = (self.y as f32) * (1f32 - t) + (other.y as f32) * t;
            Point {
                x: (x - (self.x as f32)).trunc() as i64 + self.x,
                y: (y - (self.y as f32)).trunc() as i64 + self.y
            }
        } else {
            *other
        }
    }
}
