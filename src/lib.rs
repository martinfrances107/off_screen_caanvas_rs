use std::f64;
use wasm_bindgen::prelude::*;

use web_sys::Path2d;

#[wasm_bindgen(start)]
fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let onscreen_canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let onscreen_context = onscreen_canvas
        .get_context("bitmaprenderer")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::ImageBitmapRenderingContext>()
        .unwrap();

    let off_canvas: web_sys::OffscreenCanvas =
        web_sys::OffscreenCanvas::new(800, 600).expect("allocation of OffscreenCanvas failed");

    let context = off_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
        .unwrap();
    context.begin_path();

    // Draw the outer circle.
    let path_face = Path2d::new().unwrap();
    path_face
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    let path_mouth = Path2d::new().unwrap();
    path_mouth.move_to(110.0, 75.0);
    path_mouth
        .arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)
        .unwrap();

    // Draw the left eye.
    let path_l_eye = Path2d::new().unwrap();
    path_l_eye.move_to(65.0, 65.0);
    path_l_eye
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    let path_r_eye = Path2d::new().unwrap();
    path_r_eye.move_to(95.0, 65.0);
    path_r_eye
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    path_face.add_path(&path_mouth);
    path_face.add_path(&path_l_eye);
    path_face.add_path(&path_r_eye);
    context.stroke_with_path(&path_face);
    // context.stroke();

    let bitmap = off_canvas.transfer_to_image_bitmap().unwrap();

    onscreen_context.transfer_from_image_bitmap(&bitmap);
}
