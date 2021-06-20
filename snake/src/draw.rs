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
                  ctx: &mut Context,
                  mode: DrawMode)
{
    let actual_x = to_coord(game_x);
    let actual_y = to_coord(game_y);

    let r = graphics::Rect::new(actual_x, actual_y, to_coord(width), to_coord(height));
    let mesh_r = Mesh::new_rectangle(ctx, mode, r, color).expect("Error making rectangle mesh.");

    graphics::draw(ctx, &mesh_r, DrawParam::default()).expect("Error trying to draw rectangle mesh.");
}

pub fn draw_block(game_x: i32, game_y: i32, color: Color, ctx: &mut Context)
{
    draw_rect(game_x, game_y, 1, 1, color, ctx, DrawMode::stroke(5.0));
}

pub fn translate(x: f32, y: f32, ctx: &mut Context)
{
    let translation_matrix = DrawParam::new().dest([x, y]).to_matrix();
    graphics::push_transform(ctx, Some(translation_matrix));
    graphics::apply_transformations(ctx).expect("Error applying transformation.");
}

pub fn reset_translate(ctx: &mut Context)
{
    translate(0.0, 0.0, ctx);
}
