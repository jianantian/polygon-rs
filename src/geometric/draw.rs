use super::{ConvexPolygon, SimplePolygon};
use crate::base::Color;
use geo::Point;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::JsValue;

pub trait Draw {
    fn draw(&self, _ctx: Rc<web_sys::CanvasRenderingContext2d>, _color: &Color, _fill: bool) {}

    fn stroke(&self, _ctx: Rc<web_sys::CanvasRenderingContext2d>, _color: &Color) {
        self.draw(_ctx,  _color, false)
    }

    fn fill(&self, _ctx: Rc<web_sys::CanvasRenderingContext2d>, _color: &Color) {
        self.draw(_ctx,  _color, true)
    }
}

pub fn draw_line(ctx: Rc<web_sys::CanvasRenderingContext2d>, start: (f64, f64), end: (f64, f64)) {
    ctx.begin_path();
    ctx.move_to(start.0, start.1);
    ctx.line_to(end.0, end.1);
}

impl Draw for Vec<(f64, f64)> {
    fn draw(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, color: &Color, fill: bool) {
        if self.is_empty() {
            return;
        }
        let color_str = JsValue::from_str(&color.to_string());
        ctx.set_fill_style(&color_str);
        ctx.set_stroke_style(&color_str);
        ctx.begin_path();
        let num = self.len();
        let start = self[0];
        ctx.move_to(start.0, start.1);
        for i in 1..num {
            let pt = self[i];
            ctx.line_to(pt.0, pt.1);
        }
        if num >= 3 {
            ctx.line_to(start.0, start.1);
            ctx.stroke();
            if fill {
                ctx.fill();
            }
        }
    }
}

impl Draw for Vec<Point<f64>> {
    fn draw(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, color: &Color, fill: bool) {
        if self.is_empty() {
            return;
        }
        let color_str = JsValue::from_str(&color.to_string());
        ctx.set_fill_style(&color_str);
        ctx.set_stroke_style(&color_str);
        ctx.begin_path();
        let num = self.len();
        let start = self[0];
        ctx.move_to(start.x(), start.y());
        for i in 1..num {
            let pt = self[i];
            ctx.line_to(pt.x(), pt.y());
        }
        if num >= 3 {
            ctx.line_to(start.x(), start.y());
            ctx.stroke();
            if fill {
                ctx.fill();
            }
        }
    }
}

impl Draw for Vec<Vec<Point<f64>>> {
    fn draw(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, color: &Color, fill: bool) {
        if self.is_empty() {
            return;
        }
        let color_str = JsValue::from_str(&color.to_string());
        ctx.set_fill_style(&color_str);
        ctx.set_stroke_style(&color_str);
        ctx.begin_path();

        for pts in self.iter() {
            let num = pts.len();
            let start = pts[0];
            ctx.move_to(start.x(), start.y());
            for i in 1..num {
                let pt = pts[i];
                ctx.line_to(pt.x(), pt.y());
            }
            if num >= 3 {
                ctx.line_to(start.x(), start.y());
            }
        }
        ctx.stroke();
        if fill {
            ctx.fill();
        }
    }
}

impl Draw for SimplePolygon {
    fn draw(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, color: &Color, fill: bool) {
        self.vertices().draw(ctx, color, fill);
    }
}

impl Draw for ConvexPolygon {
    fn draw(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, color: &Color, fill: bool) {
        self.vertices().draw(ctx, color, fill);
    }
}
