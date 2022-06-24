use crate::vec2;
use std::f32::consts::PI;

pub fn sine_wave(t: f32, p: f32, a: f32, mid: f32) -> f32 {
    return (t * 2.0 * PI / p).sin() * a * mid;
}

pub fn sine_btwn(t: f32, p: f32, min: f32, max: f32) -> f32 {
    let mut mid = (max - min) / 2.0;
    let a = max - min;
    return sine_wave(t, p, a, mid);
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn mult(&mut self, n: f32) -> Vec2 {
        Vec2::new(self.x * n, self.y * n)
    }
}
impl std::ops::Mul for Vec2 {
    type Output = Self;
    fn mul(self, other: Vec2) -> Self::Output {
        return vec2!(self.x * other.x, self.y * other.y);
    }
}
impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Vec2) -> Self::Output {
        return vec2!(self.x + other.x, self.y + other.y);
    }
}
impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self::Output {
        return vec2!(self.x - other.x, self.y - other.y);
    }
}
impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}
impl std::ops::Div for Vec2 {
    type Output = Self;
    fn div(self, other: Vec2) -> Self::Output {
        return vec2!(self.x / other.x, self.y / other.y);
    }
}
