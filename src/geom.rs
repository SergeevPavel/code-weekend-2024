#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn dst_sqr(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn is_in_radius_of(&self, other: &Point, r: i32) -> bool {
        self.dst_sqr(other) <= r * r
    }

    pub fn in_radius_with_direction(&self, other: &Point, r: i32) -> Point {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let t = r as f32 / ((dx * dx + dy * dy) as f32).sqrt();
        if t <= 1f32 {
            let x = (self.x as f32) * (1f32 - t) + (other.x as f32) * t;
            let y = (self.y as f32) * (1f32 - t) + (other.y as f32) * t;
            Point {
                x: (x - (self.x as f32)).trunc() as i32 + self.x,
                y: (y - (self.y as f32)).trunc() as i32 + self.y
            }
        } else {
            *other
        }
    }
}

// impl Sub<Point> for Point {
//     type Output = Self;
//
//     fn sub(self, rhs: Point) -> Self::Output {
//         Self {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y
//         }
//     }
// }