mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord;

const BG_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main()
{
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new("Snake",
                                    [to_coord(width), to_coord(height)])
                                    .exit_on_esc(true)
                                    .build()
                                    .unwrap();
    let mut game: Game = Game::new(width, height);

    while let Some(event) = window.next()
    {
        if let Some(Button::Keyboard(key)) = event.press_args()
        {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BG_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
