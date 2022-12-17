use crate::prelude::*;
pub struct Map {
    data: Vec<u8>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            data: vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1,
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1,
            ],
        }
    }

    pub fn index(x: usize, y: usize) -> usize {
        (y * MAP_NUM_COLS as usize) + x
    }

    pub fn get(&self, x: u32, y: u32) -> u8 {
        // if x >= MAP_NUM_COLS || y >= MAP_NUM_ROWS {
        //     return 1;
        // }
        // println!("{} {}", x, y);

        self.data[Map::index(x as usize, y as usize)]
    }

    pub fn set(&mut self, x: u32, y: u32, val: u8) {
        self.data[Map::index(x as usize, y as usize)] = val;
    }

    pub fn has_wall_at(&self, x: f32, y: f32) -> bool {
        if x < 0.0 || y < 0.0 || x as u32 >= WINDOW_WIDTH || y as u32 >= WINDOW_HEIGHT {
            return true;
        }

        self.get(x as u32 / TILE_SIZE, y as u32 / TILE_SIZE) != 0
    }

    pub fn render(&self, app: &mut App) {
        for i in 0..MAP_NUM_COLS {
            for j in 0..MAP_NUM_ROWS {
                let tile_color = if self.get(i, j) != 0 { 255 } else { 0 };

                let tile_x: i32 = (i * TILE_SIZE) as i32;
                let tile_y: i32 = (j * TILE_SIZE) as i32;
                let scaled_tile_x = (tile_x as f32 * MINIMAP_SCALE_FACTOR) as i32;
                let scaled_tile_y = (tile_y as f32 * MINIMAP_SCALE_FACTOR) as i32;
                let scaled_tile_size = (TILE_SIZE as f32 * MINIMAP_SCALE_FACTOR) as u32;

                app.renderer
                    .set_draw_color(Color::RGBA(tile_color, tile_color, tile_color, 255));
                let map_tile_rect = Rect::new(
                    scaled_tile_x,
                    scaled_tile_y,
                    scaled_tile_size,
                    scaled_tile_size,
                );

                app.renderer.fill_rect(map_tile_rect).unwrap();
            }
        }
    }
}
