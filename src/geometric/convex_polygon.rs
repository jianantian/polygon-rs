use super::utils::to_point_list;
use geo::area::Area;
use geo::{LineString, Point, Polygon};
use std::fmt;

// 简单多边形
// 连通
#[derive(PartialEq, Clone, Debug)]
pub struct ConvexPolygon {
    pub(crate) _base_polygon: Polygon<f64>,
}

impl ConvexPolygon {
    pub fn new(ex: LineString<f64>) -> Self {
        ConvexPolygon {
            _base_polygon: Polygon::new(ex, vec![]),
        }
    }

    fn exterior(&self) -> &LineString<f64> {
        self._base_polygon.exterior()
    }

    pub fn len(&self) -> usize {
        1 as usize
    }

    pub fn vertices(&self) -> Vec<Point<f64>> {
        to_point_list(self.exterior())
    }

    pub fn is_simple(&self) -> bool {
        true
    }

    pub fn has_hole(&self) -> bool {
        false
    }

    pub fn num_hole(&self) -> usize {
        0
    }

    pub fn holes(&self) -> Vec<Self> {
        vec![]
    }

    pub fn area(&self) -> f64 {
        self._base_polygon.area()
    }
}

impl fmt::Display for ConvexPolygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::from("");
        let pts = self.vertices();
        for pt in &pts[..pts.len() - 1]  {
            res.push_str(&format!("({}, {}), ", pt.x(), pt.y()))
        }
        res.truncate(res.len() - 2);
        write!(f, "ConvexPolygon: [{}]", res)
    }
}
