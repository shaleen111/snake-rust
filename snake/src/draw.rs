use ggez::{Context, graphics, graphics::Color, graphics::Mesh,
           graphics::DrawMode, graphics::DrawParam};

const SCALE_FACTOR: f32 = 25.0;

pub fn to_coord(game_coord: i32) -> f32
{
    (game_coord as f32) * SCALE_FACTOR
}

pub fn draw_rect( game_x: i32,
                  game_y: i32,
                  width: i32,
                  height: i32,
                  color: Color,
                  ctx: &mut Context)
{
    let actual_x: f32 = to_coord(game_x);
    let actual_y: f32 = to_coord(game_y);

    let r = graphics::Rect::new(actual_x, actual_y, to_coord(width), to_coord(height));
    let mesh_r = Mesh::new_rectangle(ctx, DrawMode::fill(), r, color).expect("Error making rectangle mesh.");

    graphics::draw(ctx, &mesh_r, DrawParam::default()).expect("Error trying to draw rectangle mesh.");
}

pub fn draw_block(game_x: i32, game_y: i32, color: Color, ctx: &mut Context)
{
    draw_rect(game_x, game_y, 1, 1, color, ctx);
}
