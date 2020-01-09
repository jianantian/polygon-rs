use super::utils::to_point_list;
use super::{ConvexPolygon, Draw, SimplePolygon};
use crate::base::Color;
use geo::area::Area;
use geo::{LineString, Point, Polygon};
use std::rc::Rc;
use wasm_bindgen::JsValue;

// enum GeneralPolygonBase {
//     Multi(MultiPolygon<f64>),
//     Single(Polygon<f64>),
// }

// 多边形
// 连通
#[derive(PartialEq, Clone, Debug)]
pub struct SinglePolygon {
    pub(crate) _base_polygon: Polygon<f64>,
}

impl SinglePolygon {
    pub fn new(ex: LineString<f64>, inners: Vec<LineString<f64>>) -> Self {
        SinglePolygon {
            _base_polygon: Polygon::new(ex, inners),
        }
    }

    fn exterior(&self) -> &LineString<f64> {
        self._base_polygon.exterior()
    }

    fn interior(&self) -> &[LineString<f64>] {
        self._base_polygon.interiors()
    }

    pub fn len(&self) -> usize {
        1 as usize
    }

    pub fn vertices(&self) -> Vec<Point<f64>> {
        to_point_list(self.exterior())
    }

    pub fn has_hole(&self) -> bool {
        self.interior().len() > 0
    }

    pub fn num_hole(&self) -> usize {
        self.interior().len()
    }

    pub fn out_polygon(&self) -> SimplePolygon {
        SimplePolygon::new(self.exterior().clone())
    }

    pub fn holes(&self) -> Vec<SimplePolygon> {
        self.interior()
            .into_iter()
            .map(|line| SimplePolygon::new(line.clone()))
            .collect()
    }

    pub fn hole(&self, i: usize) -> Option<SimplePolygon> {
        match self.interior().get(i) {
            None => None,
            Some(line) => Some(SimplePolygon::new(line.clone())),
        }
    }

    pub fn is_simple(&self) -> bool {
        self.interior().len() == 0
    }

    pub fn is_convex(&self) -> bool {
        self.is_simple() && self._base_polygon.is_convex()
    }

    pub fn area(&self) -> f64 {
        self._base_polygon.area()
    }
}

impl From<Polygon<f64>> for SinglePolygon {
    fn from(pol: Polygon<f64>) -> Self {
        SinglePolygon { _base_polygon: pol }
    }
}

impl From<SimplePolygon> for SinglePolygon {
    fn from(pol: SimplePolygon) -> Self {
        SinglePolygon {
            _base_polygon: pol._base_polygon,
        }
    }
}

impl From<ConvexPolygon> for SinglePolygon {
    fn from(pol: ConvexPolygon) -> Self {
        SinglePolygon {
            _base_polygon: pol._base_polygon,
        }
    }
}

impl From<SinglePolygon> for SimplePolygon {
    fn from(pol: SinglePolygon) -> Self {
        pol.out_polygon()
    }
}

impl Draw for SinglePolygon {
    fn draw(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, color: &Color, fill: bool) {
        let color_str = JsValue::from_str(&color.to_string());
        ctx.set_fill_style(&color_str);
        ctx.set_stroke_style(&color_str);
        ctx.begin_path();
        let mut start = Point::<f64>::new(0., 0.);
        let mut end = Point::<f64>::new(0., 0.);

        let mut pts = to_point_list(self.exterior());
        for (i, pt) in pts.iter().enumerate() {
            if i == 0 {
                start = pt.clone();
                ctx.move_to(start.x(), start.y());
            } else if i == pts.len() - 1 {
                end = pt.clone();
            } else {
                ctx.line_to(pt.x(), pt.y());
            }
        }
        if start != end {
            ctx.line_to(start.x(), start.y());
        }

        for lines in self.interior().iter() {
            pts = to_point_list(lines);
            for (i, pt) in pts.iter().enumerate() {
                ctx.line_to(pt.x(), pt.y());
                if i == 0 {
                    start = pt.clone();
                } else if i == pts.len() - 1 {
                    end = pt.clone();
                }
            }

            if start != end {
                ctx.line_to(start.x(), start.y());
            }
        }

        ctx.stroke();
        if fill {
            ctx.fill();
        }
    }
}

#[cfg(test)]
mod test {
    use super::SinglePolygon;
    use crate::geometric::FromPoints;
    #[test]
    fn test_single_polygon() {
        let poly = SinglePolygon::from_points(
            vec![
                (0.0, 0.0),
                (4.0, 0.0),
                (4.0, 1.0),
                (1.0, 1.0),
                (1.0, 4.0),
                (0.0, 4.0),
                (0.0, 0.0),
            ],
            vec![],
        );
        assert_eq!(poly.area(), 7.0f64);
    }
}
