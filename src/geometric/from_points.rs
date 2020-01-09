use super::{SimplePolygon, SinglePolygon};
use geo::{LineString, Point};

pub trait FromPoints<T> {
    fn from_points(_ex: T, _inner: Vec<T>) -> Self;
}

pub trait FromPoint<T> {
    fn from_points(_ex: T) -> Self;
}

impl FromPoints<Vec<Point<f64>>> for SinglePolygon {
    fn from_points(ex: Vec<Point<f64>>, inner: Vec<Vec<Point<f64>>>) -> Self {
        SinglePolygon::new(
            LineString::from(ex),
            inner
                .into_iter()
                .map(|_pts| LineString::from(_pts))
                .collect(),
        )
    }
}

impl FromPoints<Vec<(f64, f64)>> for SinglePolygon {
    fn from_points(ex: Vec<(f64, f64)>, inner: Vec<Vec<(f64, f64)>>) -> Self {
        SinglePolygon::new(
            LineString::from(ex),
            inner
                .into_iter()
                .map(|_pts| LineString::from(_pts))
                .collect(),
        )
    }
}

impl FromPoint<Vec<Point<f64>>> for SimplePolygon {
    fn from_points(ex: Vec<Point<f64>>) -> Self {
        SimplePolygon::new(LineString::from(ex))
    }
}

impl FromPoint<Vec<(f64, f64)>> for SimplePolygon {
    fn from_points(ex: Vec<(f64, f64)>) -> Self {
        SimplePolygon::new(LineString::from(ex))
    }
}
