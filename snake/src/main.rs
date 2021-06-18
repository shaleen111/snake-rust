// mod draw;
// mod snake;
// mod game;

// use piston_window::*;
// use piston_window::types::Color;

// use crate::game::Game;
// use crate::draw::to_coord;


// fn main()
// {
    // let (width, height) = (20, 20);

    // let mut window: PistonWindow = WindowSettings::new("Snake",
    //                                 [to_coord(width), to_coord(height)])
    //                                 .exit_on_esc(true)
    //                                 .build()
    //                                 .unwrap();
    // let mut game: Game = Game::new(width, height);

    // while let Some(event) = window.next()
    // {
        //     if let Some(Button::Keyboard(key)) = event.press_args()
        //     {
            //         game.key_pressed(key);
            //     }

            //     window.draw_2d(&event, |c, g, _| {
                //         clear(BG_COLOR, g);
                //         game.draw(&c, g);
                //     });

                //     event.update(|arg| {
                    //         game.update(arg.dt);
                    //     });
                    // }

                    // }

mod draw;

use ggez::{ContextBuilder, event, conf,
           graphics, graphics::Color};

use crate::draw::{to_coord, draw_block};

const BG_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);

struct WindowState
{
    game: (),
}

impl event::EventHandler for WindowState
{
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult
    {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult
    {
        graphics::clear(ctx, BG_COLOR);

        draw_block(0, 0, Color::new(1.0, 0.0, 0.0, 1.0), ctx);

        graphics::present(ctx).expect("Error presenting.");
        Ok(())
    }
}

fn main()
{
    let (width, height) = (20, 20);
    let (ctx, event_loop) = ContextBuilder::new("Snake", "Shaleen Baral")
                            .window_mode(conf::WindowMode::default().dimensions(to_coord(width) as f32, to_coord(height) as f32))
                            .build()
                            .expect("Error building context.");

    let window_state = WindowState{game: ()};

    ggez::event::run(ctx, event_loop, window_state);
}
