mod draw;
mod snake;
mod game;

use ggez::{ContextBuilder, event, conf,
           graphics, graphics::Color,
           input::keyboard,
           timer};

use crate::draw::{to_coord, draw_block};
use crate::game::Game;

const BG_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);

struct WindowState
{
    game: Game,
}

impl event::EventHandler for WindowState
{
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult
    {
        // self.game.update(timer::delta(ctx).as_secs_f64());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult
    {
        graphics::clear(ctx, BG_COLOR);

        // self.game.draw(ctx);
        draw_block(3, 4, Color::new(1.0, 0.0, 0.0, 1.0), ctx);

        graphics::present(ctx).expect("Error presenting.");
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut ggez::Context, keycode: keyboard::KeyCode,
                      _keymods: keyboard::KeyMods, repeat: bool)
    {
        if keycode == keyboard::KeyCode::Escape
        {
            event::quit(ctx);
        }
        // self.game.key_pressed(keycode, repeat);
    }

    fn resize_event(&mut self,_ctx: &mut ggez::Context, _width: f32, _height: f32)
    {

    }
}

fn main()
{
    let (width, height) = (20, 20);
    let (ctx, event_loop) = ContextBuilder::new("Snake", "Shaleen Baral")
                            .window_mode(conf::WindowMode::default().dimensions(to_coord(width), to_coord(height)))
                            .build()
                            .expect("Error building context.");

    let window_state = WindowState{game: Game::new(width, height)};

    ggez::event::run(ctx, event_loop, window_state);
}
