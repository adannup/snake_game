const BLOCK_SIZE: f64 = 25.0;

fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_window(width: i32, height: i32) -> [u32; 2] {
    [to_coord_u32(width), to_coord_u32(height)]
}
