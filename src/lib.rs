extern crate console_error_panic_hook;

mod snake_game;

use snake_game::render;
use snake_game::SnakeGame;
use std::cell::RefCell;
use std::f64;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

fn window() -> web_sys::Window {
    web_sys::window().expect("No global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

fn get_canvas() -> web_sys::HtmlCanvasElement {
    let canvas = document().get_element_by_id("canvas").unwrap();
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

fn get_canvas_context() -> web_sys::CanvasRenderingContext2d {
    get_canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub fn happy_face() {
    let document = document();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}

#[wasm_bindgen]
pub fn count() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::new(move || {
        let p = document().get_element_by_id("render-text").unwrap();
        let p: web_sys::Node = p.dyn_into::<web_sys::Node>().map_err(|_| ()).unwrap();

        if i > 300 {
            p.set_text_content(Some("All Done"));

            // Drop f
            let _ = f.borrow_mut().take();
            return;
        }

        i += 1;
        let text = format!("requestAnimationFrame has been called {} times", i);
        p.set_text_content(Some(&text));

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Print debug info to the console
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    //count()?;
    //happy_face();

    //let snake = SnakeGame::new(128, 128);
    let mut snake = SnakeGame::new(16, 16);
    //let _ = snake.key_press();

    let _ = render(snake);

    Ok(())
}
