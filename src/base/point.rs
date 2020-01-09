use super::Vector2;
use super::EPSILON;
use std::fmt;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Point2 {
    x: f64,
    y: f64,
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Self {
        Point2 { x, y }
    }

    pub fn zero() -> Self {
        Point2 { x: 0f64, y: 0f64 }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn from(&self, start_pt: &Point2) -> Vector2 {
        Vector2::new(self.x - start_pt.x, self.y - start_pt.y)
    }

    pub fn to(&self, end_pt: &Point2) -> Vector2 {
        Vector2::new(end_pt.x - self.x, end_pt.y - self.y)
    }

    pub fn move_along(&self, v: &Vector2) -> Point2 {
        Point2 {
            x: self.x + v.x(),
            y: self.y + v.y(),
        }
    }

    pub fn square_euclid_distance(&self, pt: &Point2) -> f64 {
        (self.x - pt.x).powi(2) + (self.y - pt.y).powi(2)
    }

    pub fn euclid_distance(&self, pt: &Point2) -> f64 {
        self.square_euclid_distance(pt).sqrt()
    }

    pub fn manhattan_distance(&self, pt: &Point2) -> f64 {
        (self.x - pt.x).abs() + (self.y - pt.y).abs()
    }
}

// 逆时针
pub fn is_ccw(pt_a: &Point2, pt_b: &Point2, pt_c: &Point2) -> bool {
    pt_b.from(pt_a).exterior(&pt_c.from(pt_a)) > EPSILON
}

pub fn square_euclid_distance(pt_1: &Point2, pt_2: &Point2) -> f64 {
    pt_1.square_euclid_distance(pt_2)
}

pub fn euclid_distance(pt_1: &Point2, pt_2: &Point2) -> f64 {
    pt_1.euclid_distance(pt_2)
}

pub fn manhattan_distance(pt_1: &Point2, pt_2: &Point2) -> f64 {
    pt_1.manhattan_distance(pt_2)
}

impl PartialOrd for Point2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(Ordering::Equal) => self.y.partial_cmp(&other.y),
            res => res,
        }
    }
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
