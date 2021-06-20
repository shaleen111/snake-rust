use ggez::{Context, graphics::Color, graphics::DrawMode, input::keyboard};

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rect, translate, reset_translate};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0);
const BORDER_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.8);
const GAMEOVER_COLOR: Color = Color::new(1.0, 0.0, 0.0, 0.5);

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

const SHAKE_DURATION: f64 = 0.25;
const SHAKE_MAGNITUDE: f32 = 3.0;

pub struct Game
{
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    shake_time: f64,
    shake_screen: bool,

    game_over: bool,
    waiting_time: f64,
}

impl Game
{
    pub fn new(width: i32, height: i32) -> Game
    {
        Game
        {
            snake: Snake::new(2, 2),

            food_exists: true,
            food_x: 6,
            food_y: 4,

            width,
            height,

            shake_time: 0.0,
            shake_screen: false,

            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: keyboard::KeyCode, repeat: bool)
    {
        if self.game_over | repeat
        {
            return;
        }

        let dir = match key
        {
            keyboard::KeyCode::W | keyboard::KeyCode::Up => Some(Direction::UP),
            keyboard::KeyCode::S | keyboard::KeyCode::Down => Some(Direction::DOWN),
            keyboard::KeyCode::A | keyboard::KeyCode::Left => Some(Direction::LEFT),
            keyboard::KeyCode::D | keyboard::KeyCode::Right => Some(Direction::RIGHT),
            _ => None,
        };

        if let Some(d) = dir
        {
            if d == self.snake.facing().opposite()
            {
                return;
            }
        }
        else
        {
            return
        }

        self.update_snake(dir);
    }

    pub fn draw(&mut self, ctx: &mut Context)
    {
        if self.shake_screen
        {
            if self.shake_time < SHAKE_DURATION
            {
                let mut rng = thread_rng();

                let dx = rng.gen_range(-SHAKE_MAGNITUDE..=SHAKE_MAGNITUDE);
                let dy = rng.gen_range(-SHAKE_MAGNITUDE..=SHAKE_MAGNITUDE);

                translate(ctx, dx, dy);
            }
            else
            {
                reset_translate(ctx);
                self.shake_screen = false;
                self.shake_time = 0.0;
            }
        }

        self.snake.draw(ctx);

        if self.food_exists
        {
            draw_block(self.food_x, self.food_y, FOOD_COLOR, ctx);
        }

        draw_rect(0, 0, self.width, 1, BORDER_COLOR, ctx, DrawMode::fill());
        draw_rect(0, 1, 1, self.height - 1, BORDER_COLOR, ctx, DrawMode::fill());
        draw_rect(self.width - 1, 1, 1, self.height - 1, BORDER_COLOR, ctx, DrawMode::fill());
        draw_rect(1, self.height - 1, self.width - 2, 1, BORDER_COLOR, ctx, DrawMode::fill());

        if self.game_over
        {
            draw_rect(0, 0, self.width, self.height, GAMEOVER_COLOR, ctx, DrawMode::fill());
        }
    }

    pub fn update(&mut self, dt: f64)
    {
        self.waiting_time += dt;

        if self.game_over
        {
            if self.waiting_time > RESTART_TIME
            {
                self.restart();
            }
            return;
        }

        if !self.food_exists
        {
            self.shake_screen = true;
            self.add_food();
        }

        if self.shake_screen
        {
            self.shake_time += dt;
        }

        if self.waiting_time > MOVING_PERIOD
        {
            self.update_snake(None);
        }
    }

    pub fn eat(&mut self)
    {
        let (front_x, front_y) = self.snake.head();

        if front_x == self.food_x && front_y == self.food_y
        {
            self.food_exists = false;
            self.snake.extend_tail();
        }
    }

    pub fn is_snake_alive(&self, dir: Option<Direction>) -> bool
    {
        let (next_x, next_y) = self.snake.next_direction(dir);

        !self.snake.overlap_except_tail(next_x, next_y) &&
        next_x > 0 &&
        next_x < self.width - 1 &&
        next_y > 0 &&
        next_y < self.height - 1
    }

    pub fn add_food(&mut self)
    {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..(self.width - 1));
        let mut new_y = rng.gen_range(1..(self.height - 1));

        while self.snake.overlap_except_tail(new_x, new_y)
        {
            new_x = rng.gen_range(1..(self.width - 1));
            new_y = rng.gen_range(1..(self.height - 1));
        }

        self.food_exists = true;
        self.food_x = new_x;
        self.food_y = new_y;
    }

    pub fn update_snake(&mut self, dir: Option<Direction>)
    {
        if self.is_snake_alive(dir)
        {
            self.snake.move_forward(dir);
            self.eat();
        }
        else
        {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    pub fn restart(&mut self)
    {
        self.snake = Snake::new(2,2);
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
        self.waiting_time = 0.0;
    }
}
