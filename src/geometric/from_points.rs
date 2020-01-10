use super::{SimplePolygon, SinglePolygon};
use geo::{LineString, Point};
use itertools::Itertools;

pub trait FromPoints<T> {
    fn from_points(_ex: T, _inner: Vec<T>) -> Self;
}

pub trait FromPoint<T> {
    fn from_points(_ex: T) -> Self;
}

fn get_pts(values: Vec<f64>) -> Vec<(f64, f64)> {
    values.into_iter().tuples::<(_, _)>().collect()
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

impl FromPoints<Vec<f64>> for SinglePolygon {
    fn from_points(ex: Vec<f64>, inner: Vec<Vec<f64>>) -> Self {
        SinglePolygon::new(
            LineString::from(get_pts(ex)),
            inner
                .into_iter()
                .map(|_pts| LineString::from(get_pts(_pts)))
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

impl FromPoint<Vec<f64>> for SimplePolygon {
    fn from_points(ex: Vec<f64>) -> Self {
        SimplePolygon::new(LineString::from(get_pts(ex)))
    }
}

impl FromPoint<&Vec<Point<f64>>> for SimplePolygon {
    fn from_points(ex: &Vec<Point<f64>>) -> Self {
        SimplePolygon::new(LineString::from(ex.clone()))
    }
}

impl FromPoint<&Vec<(f64, f64)>> for SimplePolygon {
    fn from_points(ex: &Vec<(f64, f64)>) -> Self {
        SimplePolygon::new(LineString::from(ex.clone()))
    }
}

impl FromPoint<&Vec<f64>> for SimplePolygon {
    fn from_points(ex: &Vec<f64>) -> Self {
        SimplePolygon::new(LineString::from(get_pts(ex.clone())))
    }
}
