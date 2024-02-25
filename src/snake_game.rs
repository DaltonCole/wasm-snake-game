use crate::document;
use crate::get_canvas;
use crate::get_canvas_context;
use crate::request_animation_frame;
use crate::window;
use js_sys::Math::random;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::VecDeque;
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
    snake: VecDeque<Coordinate>,
    direction: Direction,
}

impl Default for SnakeGame {
    fn default() -> Self {
        SnakeGame::new(128, 128)
    }
}

impl SnakeGame {
    pub fn new(width: u16, height: u16) -> SnakeGame {
        let mut snake = SnakeGame {
            width,
            height,
            //food: None,
            food: None,
            snake: VecDeque::from([
                Coordinate::new(0, 0),
                Coordinate::new(1, 0),
                Coordinate::new(2, 0),
            ]),
            direction: Direction::East,
        };

        Self::grow_food(&mut snake);
        snake
    }

    fn get_all_possible_food_locations(&self) -> Vec<Coordinate> {
        todo!()
    }

    fn grow_food(snake: &mut SnakeGame) {
        let x = (random() * snake.width as f64) as u16;
        let y = (random() * snake.height as f64) as u16;

        snake.food = Some(Coordinate::new(x, y));
    }

    /// Moves the snake
    ///
    /// Returns:
    ///     True if the snake successfully moved
    ///     False if the snake ran into its self and died
    pub fn move_snake(&mut self) -> bool {
        // Get snakes head
        let head = self.snake.back().unwrap();

        // Find new head position
        let new_head = match self.direction {
            Direction::East => {
                // Go Right
                Coordinate::new((head.x + 1) % self.width, head.y)
            }
            Direction::West => {
                // Go left
                Coordinate::new((head.x + self.width - 1) % self.width, head.y)
            }
            Direction::North => {
                // Go Up
                Coordinate::new(head.x, (head.y + self.height - 1) % self.height)
            }
            Direction::South => {
                // Go Right
                Coordinate::new(head.x, (head.y + 1) % self.height)
            }
        };

        // See if we're over the food
        if new_head == self.food.unwrap() {
            Self::grow_food(self);
        } else {
            self.snake.pop_front();
        }
        self.snake.push_back(new_head);

        // See if we've consumed ourselves
        for i in 0..self.snake.len() {
            for j in 0..self.snake.len() {
                if i != j && self.snake[i] == self.snake[j] {
                    return false;
                }
            }
        }

        true
    }

    pub fn reset(&mut self) {
        self.snake = VecDeque::from([
            Coordinate::new(0, 0),
            Coordinate::new(1, 0),
            Coordinate::new(2, 0),
        ]);
        self.direction = Direction::East;

        Self::grow_food(self);
    }

    pub fn get_score(&self) -> usize {
        self.snake.len()
    }

    pub fn snake_head(&self) -> Coordinate {
        self.snake.back().unwrap().clone()
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
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            match event.key_code() {
                37 => {
                    // Left key press
                    snake.lock().unwrap().direction = Direction::West;
                }
                38 => {
                    // Up key press
                    snake.lock().unwrap().direction = Direction::North;
                }
                39 => {
                    // Right key press
                    snake.lock().unwrap().direction = Direction::East;
                }
                40 => {
                    // Down key press
                    snake.lock().unwrap().direction = Direction::South;
                }
                _ => {
                    console::log_1(&format!("{:?}", event.value_of()).into());
                }
            }
            console::log_1(&format!("Direction: {:?}", snake.lock().unwrap().direction).into());
        });
        window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Restart Game Button
    let button = document().get_element_by_id("restart").unwrap();
    let button = button
        .dyn_into::<web_sys::HtmlButtonElement>()
        .map_err(|_| ())
        .unwrap();
    {
        let snake = snake.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            snake.lock().unwrap().reset();
        });

        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    /*
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
    */

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
        //console::log_1(&format!("Tick #{}", i).into());

        // Move the snake
        let game_over = !our_snake.lock().unwrap().move_snake();

        console::log_1(&format!("Snake head: {:?}", our_snake.lock().unwrap().snake_head()).into());

        if game_over {
            our_snake.lock().unwrap().reset();
        }

        // Update the score
        let score = document().get_element_by_id("score").unwrap();
        let score: web_sys::Node = score.dyn_into::<web_sys::Node>().map_err(|_| ()).unwrap();
        let current_score = our_snake.lock().unwrap().get_score();
        let text = format!("{}", current_score);
        score.set_text_content(Some(&text));

        // Update the high-score
        let high_score_element = document().get_element_by_id("high-score").unwrap();
        let high_score_element: web_sys::Node = high_score_element
            .dyn_into::<web_sys::Node>()
            .map_err(|_| ())
            .unwrap();
        let high_score = high_score_element
            .text_content()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        if current_score > high_score {
            let text = format!("{}", current_score);
            high_score_element.set_text_content(Some(&text));
        }
    }));

    //request_animation_frame(g.borrow().as_ref().unwrap());
    setInterval(g.borrow().as_ref().unwrap(), 0_250);

    Ok(())
}

pub fn render(snake: SnakeGame) -> Result<(), JsValue> {
    let snake = Rc::new(Mutex::new(snake));
    key_press(&snake);
    update_game(&snake);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    /*
    let window = window();
    let canvas = get_canvas();
    canvas.set_width(window.outer_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.outer_height().unwrap().as_f64().unwrap() as u32);
    */

    *g.borrow_mut() = Some(Closure::new(move || {
        // Constants
        let cell_size = 5;
        let width = snake.lock().unwrap().width;
        let height = snake.lock().unwrap().height;

        // Get canvas
        let context = get_canvas_context();

        // Clear canvas TODO: Clear out the snake's tail
        context.clear_rect(
            0.0,
            0.0,
            ((cell_size + 1) * width + 1) as f64,
            (((cell_size + 1) * height) + 1) as f64,
        );

        // --- Draw grid --- //
        context.begin_path();

        // Add vertical lines
        context.set_stroke_style(&JsValue::from_str("black"));
        for x in 0..=width {
            context.move_to(((x * (cell_size + 1)) + 1) as f64, 0.0);
            context.line_to(
                ((x * (cell_size + 1)) + 1) as f64,
                (((cell_size + 1) * height) + 1) as f64,
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
        let head = snake.lock().unwrap().snake.back().unwrap().clone();
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
