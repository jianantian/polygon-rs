use super::{ConvexPolygon, SimplePolygon, SinglePolygon};
use geo::convexhull::ConvexHull;

pub trait SimpleConvexHull {
    fn convex_hull(&self) -> ConvexPolygon;
}

impl SimpleConvexHull for SimplePolygon {
    fn convex_hull(&self) -> ConvexPolygon {
        ConvexPolygon {
            _base_polygon: self._base_polygon.convex_hull(),
        }
    }
}

impl SimpleConvexHull for SinglePolygon {
    fn convex_hull(&self) -> ConvexPolygon {
        ConvexPolygon {
            _base_polygon: self._base_polygon.convex_hull(),
        }
    }
}
