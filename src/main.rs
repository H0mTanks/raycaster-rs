mod app;
mod map;
mod player;

mod prelude {
    pub use sdl2::event::Event;
    pub use sdl2::keyboard::Keycode;
    pub use sdl2::pixels::Color;
    pub use sdl2::rect::Rect;
    pub use sdl2::render::Canvas;
    pub use sdl2::video::Window;
    pub use sdl2::EventPump;
    pub use sdl2::Sdl;
    pub use sdl2::TimerSubsystem;
    pub use std::f32::consts::PI;

    pub const TILE_SIZE: u32 = 64;
    pub const MAP_NUM_ROWS: u32 = 13;
    pub const MAP_NUM_COLS: u32 = 20;
    pub const MINIMAP_SCALE_FACTOR: f32 = 1.0;

    pub const WINDOW_WIDTH: u32 = MAP_NUM_COLS * TILE_SIZE;
    pub const WINDOW_HEIGHT: u32 = MAP_NUM_ROWS * TILE_SIZE;

    pub const FOV_ANGLE: f32 = 60.0 * (PI / 180.0);

    pub const WALL_STRIP_WIDTH: u32 = 1;
    pub const NUM_RAYS: u32 = WINDOW_WIDTH;

    pub const TEXTURE_WIDTH: u32 = TILE_SIZE;
    pub const TEXTURE_HEIGHT: u32 = TILE_SIZE;
    pub const NUM_TEXTURES: u32 = 8;

    pub use crate::app::App;
    pub use crate::map::Map;
    pub use crate::player::Player;
}

use prelude::*;

const int_map: [u8; (MAP_NUM_ROWS * MAP_NUM_COLS) as usize] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
    1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1,
];

fn process_input(
    event_pump: &mut EventPump,
    delta_time: f32,
    player: &mut Player,
    is_running: &mut bool,
) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => *is_running = false,
            Event::KeyDown { keycode, .. } => match keycode {
                Some(key) => {
                    if key == Keycode::Escape {
                        *is_running = false;
                    }
                    if key == Keycode::Up {
                        player.update_walk_direction(1);
                    }
                    if key == Keycode::Down {
                        player.update_walk_direction(-1);
                    }
                    if key == Keycode::Right {
                        player.update_turn_direction(1);
                    }
                    if key == Keycode::Left {
                        player.update_turn_direction(-1);
                    }
                }
                None => {}
            },
            Event::KeyUp { keycode, .. } => match keycode {
                Some(key) => {
                    if key == Keycode::Up {
                        player.update_walk_direction(0);
                    }
                    if key == Keycode::Down {
                        player.update_walk_direction(0);
                    }
                    if key == Keycode::Right {
                        player.update_turn_direction(0);
                    }
                    if key == Keycode::Left {
                        player.update_turn_direction(0);
                    }
                }
                None => {}
            },

            _ => {}
        }
    }
}

fn render(app: &mut App, map: &Map, player: &Player) {
    app.renderer.set_draw_color(Color::RGBA(0, 0, 0, 255));
    app.renderer.clear();

    // renderer.set_draw_color(Color::RGBA(155, 0, 0, 255));
    // let rect: Rect = Rect::new(0, 0, 20, 20);
    // // renderer.draw_rect(rect).unwrap();
    // renderer.fill_rect(rect).unwrap();

    map.render(app);
    player.render(app);

    app.renderer.present();
}

fn main() {
    let mut app = App::new();
    let mut event_pump = app.sdl_context.event_pump().unwrap();

    let mut player = Player::new();
    let map = Map::new();

    let mut last_frame_time: u32 = 0;
    while app.is_running {
        let ticks = app.timer.ticks();

        let delta_time = (ticks - last_frame_time) as f32 / 1000.0;
        last_frame_time = ticks;

        process_input(
            &mut event_pump,
            delta_time,
            &mut player,
            &mut app.is_running,
        );

        player.update(delta_time, &map);

        render(&mut app, &map, &player);
    }
}
