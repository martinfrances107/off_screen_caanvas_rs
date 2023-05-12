extern crate js_sys;
extern crate web_sys;

use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use web_sys::ImageBitmapRenderingContext;
use web_sys::OffScreenCanvasRenderingContext2d;
use web_sys::OffscreenCanvas;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("bitmaprenderer")
        .unwrap()
        .unwrap()
        .dyn_into::<ImageBitmapRenderingContext>()
        .unwrap();

    let off_canvas = OffscreenCanvas::new(800, 600)?;

    let off_canvas_context = off_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<OffScreenCanvasRenderingContext2d>()
        .unwrap();

    off_canvas_context.begin_path();

    // Draw the outer circle.
    off_canvas_context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    off_canvas_context.move_to(110.0, 75.0);
    off_canvas_context
        .arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)
        .unwrap();

    // Draw the left eye.
    off_canvas_context.move_to(65.0, 65.0);
    off_canvas_context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    off_canvas_context.move_to(95.0, 65.0);
    off_canvas_context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    off_canvas_context.stroke();

    let bitmap = off_canvas.transfer_to_image_bitmap()?;

    context.transfer_from_image_bitmap(&bitmap);

    Ok(())
}
