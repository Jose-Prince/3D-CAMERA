use crate::fileReader::load_maze;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use std::io::Result;

pub fn render(framebuffer: &mut Framebuffer, file_path: &str) -> Result<Option<(usize, usize)>> {
    let maze = load_maze(file_path)?;
    let rows = maze.len();
    let cols = maze[0].len();
    
    let block_size = std::cmp::min(framebuffer.get_width() / cols, framebuffer.get_height() / rows);
    let mut player_position = None;

    for row in 0..rows {
        for col in 0..cols {
            if maze[row][col] == 'p' {
                let center_x = col * block_size + block_size / 2;
                let center_y = row * block_size + block_size / 2;
                player_position = Some((center_x, center_y));
            }
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    Ok(player_position)
}
