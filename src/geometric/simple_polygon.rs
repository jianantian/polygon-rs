use super::utils::to_point_list;
use super::ConvexPolygon;
use geo::area::Area;
use geo::{LineString, Point, Polygon};
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeSeq;

// use serde::{Deserialize, Serialize};

// 简单多边形
// 连通
#[derive(PartialEq, Clone, Debug)]
pub struct SimplePolygon {
    pub(crate) _base_polygon: Polygon<f64>,
}

impl SimplePolygon {
    pub fn new(ex: LineString<f64>) -> Self {
        SimplePolygon {
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

    pub fn is_simple(&self) -> bool {
        true
    }

    pub fn is_convex(&self) -> bool {
        self._base_polygon.is_convex()
    }
}

impl From<ConvexPolygon> for SimplePolygon {
    fn from(pol: ConvexPolygon) -> Self {
        SimplePolygon {
            _base_polygon: pol._base_polygon,
        }
    }
}

impl fmt::Display for SimplePolygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::from("");
        let pts = self.vertices();
        for pt in &pts[..pts.len() - 1] {
            res.push_str(&format!("({}, {}), ", pt.x(), pt.y()))
        }
        res.truncate(res.len() - 2);
        write!(f, "SimplePolygon: [{}]", res)
    }
}

impl Serialize for SimplePolygon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pts = self.vertices();
        let mut seq = serializer.serialize_seq(Some(pts.len()))?;
        for pt in self.vertices().iter() {
            seq.serialize_element(&pt.x())?;
            seq.serialize_element(&pt.y())?;
        }
        seq.end()
    }
}
