use std::collections::LinkedList;
use ggez::{Context, graphics::Color};

use crate::draw::draw_block;

const SNAKE_COLOR: Color = Color::new(0.0, 1.0, 0.0, 1.0);

#[derive(Copy, Clone, PartialEq)]
pub enum Direction
{
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction
{
    pub fn opposite(&self) -> Direction
    {
        match self
        {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
}

#[derive(Clone, Debug)]
struct Block
{
    x: i32,
    y: i32,
}

pub struct Snake
{
    body: LinkedList<Block>,
    dir: Direction,
    tail: Option<Block>,
}

impl Snake
{
    // Creates a new Snake object with
    // a length of 3 and
    // (x,y) being the coordinate of the
    // last block of the snake
    pub fn new(x: i32, y: i32) -> Snake
    {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block{
            x: x + 2,
            y,
        });
        body.push_back(Block{
            x: x + 1,
            y,
        });
        body.push_back(Block{
            x,
            y
        });

        Snake
        {
            body,
            dir: Direction::RIGHT,
            tail: None,
        }
    }

    pub fn draw(&self, con: &mut Context)
    {
        for b in &self.body
        {
            draw_block(b.x, b.y, SNAKE_COLOR, con);
        }
    }

    pub fn head(&self) -> (i32, i32)
    {
        let head_block  = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>)
    {
        match dir
        {
            Some(d) => self.dir = d,
            None => (),
        }

        let (front_x, front_y) = self.head();

        let new_block = match self.dir
        {
            Direction::UP => Block {
                x: front_x,
                y: front_y - 1,
            },
            Direction::DOWN => Block
            {
                x: front_x,
                y: front_y + 1,
            },
            Direction::LEFT => Block {
                x: front_x - 1,
                y: front_y,
            },
            Direction::RIGHT => Block {
                x: front_x + 1,
                y: front_y,
            },
        };

        self.body.push_front(new_block);
        self.tail = self.body.pop_back();
    }

    pub fn next_direction(&self, dir: Option<Direction>) -> (i32, i32)
    {
        let (front_x, front_y) = self.head();

        let moving_dir = match dir
        {
            Some(d) => d,
            None => self.dir,
        };

        match moving_dir
        {
            Direction::UP => (front_x, front_y - 1),
            Direction::DOWN => (front_x, front_y + 1),
            Direction::LEFT => (front_x - 1, front_y),
            Direction::RIGHT => (front_x + 1, front_y),
        }
    }

    pub fn extend_tail(&mut self)
    {
        let t = self.tail.clone().unwrap();
        self.body.push_back(t);
    }

    pub fn facing(&self) -> Direction
    {
        self.dir
    }

    pub fn overlap_except_tail(&self, x: i32, y: i32) -> bool
    {
        let mut c = 0;
        let non_tail_size = self.body.len() - 1;

        for b in &self.body
        {
            if b.x == x && b.y == y
            {
                return true;
            }

            c += 1;
            if c == non_tail_size
            {
                break;
            }
        }
        return false;
    }
}
