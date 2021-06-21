mod draw;
mod snake;
mod game;

use ggez::{conf, ContextBuilder, graphics};

use crate::draw::to_coord;
use crate::game::Game;

fn main()
{
    let (width, height) = (20, 20);
    let (mut ctx, mut event_loop) = ContextBuilder::new("Snake", "Shaleen Baral")
                            .window_mode(conf::WindowMode::default().dimensions(to_coord(width), to_coord(height)))
                            .add_resource_path("./resources/")
                            .build()
                            .expect("Error building context.");

    let mut game = Game::new(width, height, &mut ctx);

    graphics::set_window_title(&mut ctx, "Game");
    ggez::event::run(&mut ctx, &mut event_loop, &mut game).expect("Error running.");
}
