use piston_window::{rectangle, G2d, Context};
use piston_window::types::Color;

const SCALE_FACTOR: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64
{
    (game_coord as f64) * SCALE_FACTOR
}

pub fn draw_rect( game_x: i32,
                  game_y: i32,
                  width: i32,
                  height: i32,
                  color: Color,
                  con: &Context,
                  g: &mut G2d)
{
    let actual_x: f64 = to_coord(game_x);
    let actual_y: f64 = to_coord(game_y);

    rectangle(
        color,
        [actual_x, actual_y, SCALE_FACTOR * (width as f64), SCALE_FACTOR * (height as f64)],
        con.transform,
        g
    );
}

pub fn draw_block(game_x: i32, game_y: i32, color: Color, con: &Context, g: &mut G2d)
{
    draw_rect(game_x, game_y, 1, 1, color, con, g);
}
