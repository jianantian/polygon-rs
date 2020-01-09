pub mod color;
pub mod point;
pub mod vector;
pub use point::Point2;
pub use vector::Vector2;
pub use color:: Color;

pub const EPSILON: f64 = 1e-6;

pub trait AsPoint {
    fn as_point(&self) -> Point2;
}

pub trait AsVector {
    fn as_vector(&self) -> Vector2;
}

pub trait AlmostEqual {
    fn almost_equal(&self, other: &Self) -> bool;
}

impl AlmostEqual for Point2 {
    fn almost_equal(&self, pt: &Point2) -> bool {
        self.manhattan_distance(pt) < EPSILON
    }
}

impl AlmostEqual for Vector2 {
    fn almost_equal(&self, v: &Self) -> bool {
        (self - v).l1_norm() < EPSILON
    }
}

impl AsVector for Point2 {
    fn as_vector(&self) -> Vector2 {
        Vector2::new(self.x(), self.y())
    }
}

impl AsPoint for Vector2 {
    fn as_point(&self) -> Point2 {
        Point2::new(self.x(), self.y())
    }
}

#[cfg(test)]
mod test {
    use super::Point2;

    #[test]
    fn test_point_cmp() {
        let pt_0 = Point2::new(0f64, 1f64);
        let pt_1 = Point2::new(1f64, 0f64);
        let pt_2 = Point2::new(0f64, 2f64);
        assert!(pt_0 < pt_1);
        assert!(pt_1 > pt_0);
        assert!(pt_0 < pt_2);
    }
}
