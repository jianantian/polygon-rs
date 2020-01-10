pub mod base;
pub mod geometric;
mod utils;
use base::Color;
use geometric::{ConvexHull, Draw, FromPoint, SimplePolygon};
use std::cell::{Cell, RefCell};
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let pts: Vec<(f64, f64)> = vec![(100., 100.), (100., 220.), (60., 220.), (30., 260.)];
    let pol = SimplePolygon::from_points(pts);
    log(&format!(
        "The polygon is : {}, area is: {}",
        pol,
        pol.area()
    ));
}

#[wasm_bindgen]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("base-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx = Rc::new(
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap(),
    );

    // let pts: Vec<(f64, f64)> = vec![(100., 100.), (100., 220.), (60., 220.), (30., 260.)];
    // let pol = SimplePolygon::from_points(pts);
    // let color = Color::from((125, 123, 0));
    // pol.fill(ctx.clone(), &color);

    // let convex_pol = pol.convex_hull();
    // convex_pol.stroke(ctx.clone(), &color);

    draw(ctx, &canvas).unwrap();
}

#[wasm_bindgen]
pub fn clear_canvas(ctx: &web_sys::CanvasRenderingContext2d, w: f64, h: f64){
    ctx.clear_rect(0., 0., w, h);
}

pub fn draw(
    context: Rc<web_sys::CanvasRenderingContext2d>,
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<(), JsValue> {
    let color = Color::from((125, 123, 0));

    let mut start: (f64, f64) = (0., 0.);
    context.set_line_width(2.0);
    context.set_stroke_style(&JsValue::from_str(&color.to_string()));
    context.set_fill_style(&JsValue::from_str(&color.to_string()));

    let mut pts: Vec<(f64, f64)> = vec![];
    let pts_ref = Rc::new(RefCell::new(pts));
    // context.set_line_dash(&JsValue::from_serde(&(4, 2)).unwrap())?;
    let pressed = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let pts = pts_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if event.which() == 1 {
                if !pressed.get() {
                    context.begin_path();
                    pressed.set(true);
                    start = (event.offset_x() as f64, event.offset_y() as f64);
                    pts.borrow_mut().push(start);
                    context.move_to(start.0, start.1);
                }
            } else if event.which() == 3 {
                context.line_to(start.0, start.1);
                context.fill();
                pressed.set(false);
                log(&format!("The points is : {:?}", pts.borrow()));
                let pol = SimplePolygon::from_points(&*pts.borrow());
                log(&format!(
                    "The polygon is : {}, area is: {}",
                    pol,
                    pol.area()
                ));
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let pts = pts_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                let pt = (event.offset_x() as f64, event.offset_y() as f64);
                context.line_to(pt.0, pt.1);
                pts.borrow_mut().push(pt);
                context.stroke();
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
