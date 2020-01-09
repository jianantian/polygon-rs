use super::EPSILON;
use std::iter::Sum;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    pub fn zero() -> Self {
        Vector2 { x: 0f64, y: 0f64 }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn dot(&self, v: &Vector2) -> f64 {
        self.x * v.x + self.y * v.y
    }

    pub fn exterior(&self, v: &Vector2) -> f64 {
        self.x * v.y - self.y * v.x
    }

    pub fn square_l2_norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn l2_norm(&self) -> f64 {
        self.square_l2_norm().sqrt()
    }

    pub fn normalize(&self) -> Vector2 {
        let k = self.l2_norm();
        Vector2 {
            x: self.x / k,
            y: self.y / k,
        }
    }

    // 法向量, 逆时针旋转 90 度
    pub fn normal_vector(&self) -> Vector2 {
        let vec = self.normalize();
        Vector2 {
            x: -vec.y,
            y: vec.x,
        }
    }

    pub fn l1_norm(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }

    // 是否同向
    pub fn is_same_direction(self, v: &Vector2) -> bool {
        (self.dot(v) - 1f64).abs() < EPSILON
    }

    // 是否反向
    pub fn is_opposite_direction(self, v: &Vector2) -> bool {
        (self.dot(v) + 1f64).abs() < EPSILON
    }

    // 是否平行
    pub fn is_parallel(self, vec: &Vector2) -> bool {
        (self.dot(vec).abs() - 1f64).abs() < EPSILON
    }

    // 是否垂直
    pub fn is_perpendicular(self, v: &Vector2) -> bool {
        self.dot(v).abs() < EPSILON
    }

    // 是否水平
    pub fn is_horizontal(self) -> bool {
        self.is_perpendicular(&Vector2::new(0f64, 1f64))
    }

    // 是否竖直
    pub fn is_vertical(self) -> bool {
        self.is_perpendicular(&Vector2::new(1f64, 0f64))
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// use macro to reuse code
impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, v: Self) -> Self::Output {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Add for &Vector2 {
    type Output = Vector2;
    fn add(self, v: Self) -> Self::Output {
        Vector2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, v: Self) -> Self::Output {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Sub for &Vector2 {
    type Output = Vector2;
    fn sub(self, v: Self) -> Self::Output {
        Vector2 {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;
    fn mul(self, k: f64) -> Self::Output {
        Self {
            x: k * self.x,
            y: k * self.y,
        }
    }
}

impl Mul<Vector2> for f64 {
    type Output = Vector2;
    fn mul(self, v: Vector2) -> Self::Output {
        Vector2 {
            x: self * v.x,
            y: self * v.y,
        }
    }
}

impl Div<f64> for Vector2 {
    type Output = Self;
    fn div(self, k: f64) -> Self::Output {
        Self {
            x: self.x / k,
            y: self.y / k,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, v: Self) {
        self.x += v.x;
        self.y += v.y;
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, v: Self) {
        self.x -= v.x;
        self.y -= v.y;
    }
}

impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, k: f64) {
        self.x *= k;
        self.y *= k;
    }
}

impl DivAssign<f64> for Vector2 {
    fn div_assign(&mut self, k: f64) {
        self.x /= k;
        self.y /= k;
    }
}

impl Sum for Vector2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector2::zero(), |a, b| a + b)
    }
}
