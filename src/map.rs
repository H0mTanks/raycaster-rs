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
        println!("{} {}", x, y);
        self.data[Map::index(x as usize, y as usize)]
    }

    pub fn set(&mut self, x: u32, y: u32, val: u8) {
        self.data[Map::index(x as usize, y as usize)] = val;
    }

    pub fn render(&self, app: &mut App) {
        for i in 0..MAP_NUM_COLS {
            for j in 0..MAP_NUM_ROWS {
                let tile_x: i32 = (i * TILE_SIZE) as i32;
                let tile_y: i32 = (j * TILE_SIZE) as i32;
                let tile_color = if self.get(i, j) != 0 { 255 } else { 0 };

                app.renderer
                    .set_draw_color(Color::RGBA(tile_color, tile_color, tile_color, 255));
                let map_tile_rect = Rect::new(tile_x, tile_y, TILE_SIZE, TILE_SIZE);

                app.renderer.fill_rect(map_tile_rect).unwrap();
            }
        }
    }
}
