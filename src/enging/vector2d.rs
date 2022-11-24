use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32
}

#[allow(dead_code)]
impl Vec2d {
    pub fn new(x: f32, y: f32) -> Vec2d {
        Vec2d {
            x,
            y
        }
    }

    pub fn normalize_or_zero(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
    }

    pub fn normalized_or_zero(&self) -> Vec2d {
        if self.x == 0.0 && self.y == 0.0 {
            return Vec2d::new(0.0, 0.0);
        }
        let length = self.length();
        
        return Vec2d::new(self.x / length, self.y / length);
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn angle(&self) -> f32 {
        (self.y / self.length()).asin()
    }

    pub fn rotate(&mut self, angle: f32) {
        let cos = angle.cos();
        let sin = angle.sin();
        let temp_x = self.x * cos - self.y * sin;
        self.y = self.x * sin + self.y * cos;
        self.x = temp_x;
    }

    pub fn rotate_with_moved_center(&mut self, angle: f32, center: (f32, f32)) {
        let cos =  angle.cos();
        let sin = angle.sin();

        let c = *self - Vec2d::new(center.0, center.1);

        let temp_x = center.0 + c.x * cos - c.y * sin;
        self.y = center.1 + c.x * sin + c.y * cos;
        self.x = temp_x;
    }

    pub fn get_nearest_point_on_circle(&self, center: Vec2d, radius: f32) -> Vec2d {
        let x_sub = self.x - center.x;
        let y_sub = self.y - center.y;
        let expr = (x_sub.powi(2) + y_sub.powi(2)).sqrt();
        Vec2d {
            x: center.x + (radius * x_sub) / expr,
            y: center.y + (radius * y_sub) / expr
        }
    }

    pub fn is_in_cirlce(&mut self, center: Vec2d, radius: f32) -> bool {
        (self.x - center.x).powi(2) + (self.y - center.y).powi(2) <= radius.powi(2)
    }
    
    pub fn distance_to(&self, vec: &Vec2d) -> f32 {
        ((self.x - vec.x).powi(2) + (self.y - vec.y).powi(2)).sqrt()
    }
}

impl From<sdl2::rect::Point> for Vec2d {
    fn from(p: sdl2::rect::Point) -> Self {
        Self {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}

impl Into<sdl2::rect::Point> for Vec2d {
    fn into(self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(
            self.x as i32,
            self.y as i32,
        )
    }
}

impl ops::Add<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, rhs: Vec2d) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::AddAssign<Vec2d> for Vec2d {
    fn add_assign(&mut self, rhs: Vec2d) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl ops::SubAssign<Vec2d> for Vec2d {
    fn sub_assign(&mut self, rhs: Vec2d) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<f32> for Vec2d {
    type Output = Vec2d;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl ops::MulAssign<f32> for Vec2d {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Div<f32> for Vec2d {
    type Output = Vec2d;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec2d {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}