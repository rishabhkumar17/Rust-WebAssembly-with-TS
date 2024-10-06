use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spawn_index)],
        }
    }
}

#[wasm_bindgen]
pub struct World {
    pub width: usize,
    snake: Snake,
    size: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        let width: usize = 8;
        World {
            width,
            size: width * width,
            snake: Snake::new(10),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx: usize = self.snake_head_idx();
        self.snake.body[0].0 = (snake_idx + 1) % self.size;
    }
}

//wasm-pack build --target web
