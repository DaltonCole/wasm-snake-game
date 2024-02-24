use crate::get_canvas;
use crate::get_canvas_context;
use crate::request_animation_frame;
use crate::window;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::EventListener;
use web_sys::{KeyboardEvent, MouseEvent};

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);
}

#[wasm_bindgen]
pub struct IntervalHandle {
    interval_id: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Drop for IntervalHandle {
    fn drop(&mut self) {
        clearInterval(self.interval_id);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    pub fn new(x: u16, y: u16) -> Coordinate {
        Coordinate { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnakeGame {
    width: u16,
    height: u16,
    food: Option<Coordinate>,
    snake: Vec<Coordinate>,
    direction: Direction,
}

impl Default for SnakeGame {
    fn default() -> Self {
        SnakeGame::new(128, 128)
    }
}

impl SnakeGame {
    pub fn new(width: u16, height: u16) -> SnakeGame {
        SnakeGame {
            width,
            height,
            //food: None,
            food: Some(Coordinate::new(5, 5)),
            snake: vec![
                Coordinate::new(0, 0),
                Coordinate::new(1, 0),
                Coordinate::new(2, 0),
            ],
            direction: Direction::East,
        }
    }
}

/*
#[wasm_bindgen]
impl SnakeGame {

}
*/
pub fn key_press(snake: &Rc<Mutex<SnakeGame>>) -> Result<(), JsValue> {
    let canvas = get_canvas();
    let context = get_canvas_context();

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    let window = window();
    //let snake = Rc::new(Mutex::new(snake));
    // Key press
    {
        let snake = snake.clone();
        //let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            match event.key_code() {
                37 => {
                    // Left key press
                    console::log_1(
                        &format!("Old Direction: {:?}", snake.lock().unwrap().direction).into(),
                    );
                    snake.lock().unwrap().direction = Direction::West;
                }
                38 => {
                    // Up key press
                    console::log_1(
                        &format!("Old Direction: {:?}", snake.lock().unwrap().direction).into(),
                    );
                    snake.lock().unwrap().direction = Direction::North;
                }
                39 => {
                    // Right key press
                    console::log_1(
                        &format!("Old Direction: {:?}", snake.lock().unwrap().direction).into(),
                    );
                    snake.lock().unwrap().direction = Direction::East;
                }
                40 => {
                    // Down key press
                    console::log_1(
                        &format!("Old Direction: {:?}", snake.lock().unwrap().direction).into(),
                    );
                    snake.lock().unwrap().direction = Direction::South;
                }
                _ => {
                    console::log_1(&format!("{:?}", event.value_of()).into());
                }
            }
        });
        window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse down
    {
        let context = context.clone();
        let pressed = pressed.clone();
        //let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse Move
    {
        let context = context.clone();
        let pressed = pressed.clone();
        //let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse up
    {
        let context = context.clone();
        let pressed = pressed.clone();
        //let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

pub fn update_game(snake: &Rc<Mutex<SnakeGame>>) -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let our_snake = snake.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::new(move || {
        // Move f into closure so closure is not dropped
        let _ = f;
        i += 1;
        console::log_1(&format!("Tick #{}", i).into());
    }));

    //request_animation_frame(g.borrow().as_ref().unwrap());
    setInterval(g.borrow().as_ref().unwrap(), 1_000);

    Ok(())
}

pub fn render(snake: SnakeGame) -> Result<(), JsValue> {
    let snake = Rc::new(Mutex::new(snake));
    key_press(&snake);
    update_game(&snake);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        // Get canvas
        let context = get_canvas_context();

        // --- Draw grid --- //
        let cell_size = 5;
        let width = snake.lock().unwrap().width;
        let height = snake.lock().unwrap().height * cell_size;

        context.begin_path();

        // Add vertical lines
        context.set_stroke_style(&JsValue::from_str("black"));
        for x in 0..=width {
            context.move_to(((x * (cell_size + 1)) + 1) as f64, 0.0);
            context.line_to(
                ((x * (cell_size + 1)) + 1) as f64,
                ((cell_size + 1) * height + 1) as f64,
            );
        }

        // Add horizontal lines
        for x in 0..=height {
            context.move_to(0.0, ((x * (cell_size + 1)) + 1) as f64);
            context.line_to(
                ((cell_size + 1) * width + 1) as f64,
                ((x * (cell_size + 1)) + 1) as f64,
            );
        }
        context.stroke();

        // Draw snake
        context.begin_path();
        context.set_fill_style(&JsValue::from_str("green"));
        for coor in &snake.lock().unwrap().snake {
            context.fill_rect(
                (coor.x * (cell_size + 1) + 1) as f64,
                (coor.y * (cell_size + 1) + 1) as f64,
                cell_size as f64,
                cell_size as f64,
            )
        }
        context.stroke();

        // Draw snake head
        context.begin_path();
        context.set_fill_style(&JsValue::from_str("#32CD32"));
        let head = snake.lock().unwrap().snake.last().unwrap().clone();
        context.fill_rect(
            (head.x * (cell_size + 1) + 1) as f64,
            (head.y * (cell_size + 1) + 1) as f64,
            cell_size as f64,
            cell_size as f64,
        );
        context.stroke();

        // Draw food
        if let Some(food) = snake.lock().unwrap().food {
            context.begin_path();
            context.set_fill_style(&JsValue::from_str("red"));
            context.fill_rect(
                (food.x * (cell_size + 1) + 1) as f64,
                (food.y * (cell_size + 1) + 1) as f64,
                cell_size as f64,
                cell_size as f64,
            );
            context.stroke();
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
