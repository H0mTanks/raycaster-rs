use sdl2::rect::Point;

use crate::prelude::*;

pub struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    turn_direction: i32,
    walk_direction: i32,
    rotation_angle: f32,
    walk_speed: f32,
    turn_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: WINDOW_WIDTH as f32 / 2.0,
            y: WINDOW_HEIGHT as f32 / 2.0,
            width: 10.0, //TODO change height width to 1
            height: 10.0,
            turn_direction: 0,
            walk_direction: 0,
            rotation_angle: PI / 2.0,
            walk_speed: 100.0,
            turn_speed: 45.0 * (PI / 100.0),
        }
    }

    pub fn render(&self, app: &mut App) {
        app.renderer.set_draw_color(Color::RGBA(255, 255, 255, 255));

        let scaled_x = self.x * MINIMAP_SCALE_FACTOR;
        let scaled_y = self.y * MINIMAP_SCALE_FACTOR;
        let scaled_width = self.width * MINIMAP_SCALE_FACTOR;
        let scaled_height = self.height * MINIMAP_SCALE_FACTOR;

        let player_rect = Rect::new(
            (scaled_x - scaled_width / 2.0) as i32,
            (scaled_y - scaled_height / 2.0) as i32,
            scaled_width as u32,
            scaled_height as u32,
        );

        app.renderer
            .draw_line(
                Point::new(scaled_x as i32, scaled_y as i32),
                Point::new(
                    (scaled_x + f32::cos(self.rotation_angle) * 40.0) as i32,
                    (scaled_y + f32::sin(self.rotation_angle) * 40.0) as i32,
                ),
            )
            .unwrap();
        app.renderer.fill_rect(player_rect).unwrap();
    }

    pub fn update_walk_direction(&mut self, walk_direction: i32) {
        self.walk_direction = walk_direction;
    }

    pub fn update_turn_direction(&mut self, turn_direction: i32) {
        self.turn_direction = turn_direction;
    }

    pub fn update(&mut self, delta_time: f32, map: &Map) {
        self.rotation_angle += self.turn_direction as f32 * self.turn_speed * delta_time;
        let move_step = self.walk_direction as f32 * self.walk_speed * delta_time;

        let new_player_x = self.x + f32::cos(self.rotation_angle) * move_step;
        let new_player_y = self.y + f32::sin(self.rotation_angle) * move_step;

        //TODO: collisions

        self.x = new_player_x;
        self.y = new_player_y;
    }
}
