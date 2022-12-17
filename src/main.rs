mod app;
mod game_state;
mod map;
mod player;
mod ray;

mod prelude {
    pub use sdl2::event::Event;
    pub use sdl2::keyboard::Keycode;
    pub use sdl2::pixels::Color;
    pub use sdl2::rect::Point;
    pub use sdl2::rect::Rect;
    pub use sdl2::render::Canvas;
    pub use sdl2::video::Window;
    pub use sdl2::EventPump;
    pub use sdl2::Sdl;
    pub use sdl2::TimerSubsystem;

    pub use sdl2::{
        pixels::PixelFormatEnum,
        render::{Texture, TextureCreator},
        video::WindowContext,
    };
    pub use std::cell::RefCell;

    pub use std::f32::consts::PI;

    pub const TILE_SIZE: u32 = 64;
    pub const MAP_NUM_ROWS: u32 = 13;
    pub const MAP_NUM_COLS: u32 = 20;
    pub const MINIMAP_SCALE_FACTOR: f32 = 0.25;

    pub const WINDOW_WIDTH: u32 = MAP_NUM_COLS * TILE_SIZE;
    pub const WINDOW_HEIGHT: u32 = MAP_NUM_ROWS * TILE_SIZE;

    pub const FOV_ANGLE: f32 = 60.0 * (PI / 180.0);

    pub const WALL_STRIP_WIDTH: u32 = 1;
    pub const NUM_RAYS: u32 = WINDOW_WIDTH;

    pub const TEXTURE_WIDTH: u32 = TILE_SIZE;
    pub const TEXTURE_HEIGHT: u32 = TILE_SIZE;
    pub const NUM_TEXTURES: u32 = 8;

    pub use crate::app::App;
    pub use crate::game_state::GameState;
    pub use crate::map::Map;
    pub use crate::player::Player;
    pub use crate::ray::Ray;
}

use prelude::*;

fn process_input(event_pump: &mut EventPump, player: &mut Player, is_running: &mut bool) {
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

fn render_display_buffer(app: &mut App) {
    let mut texture = app.display_buffer_texture.borrow_mut();

    texture
        .update(None, app.display_buffer_raw(), (WINDOW_WIDTH * 4) as usize)
        .unwrap();

    app.renderer.copy(&texture, None, None).unwrap();
}

fn clear_display_buffer(app: &mut App) {
    app.display_buffer.fill(0xFF000000);
}

fn render(app: &mut App, game_state: &mut GameState) {
    app.renderer.set_draw_color(Color::RGBA(0, 0, 0, 255));
    app.renderer.clear();

    render_display_buffer(app);
    clear_display_buffer(app);

    game_state.map.render(app);
    game_state.player.render(app);
    Ray::render_rays(app, game_state);

    app.renderer.present();
}

fn main() {
    let mut app = App::new();
    let mut event_pump = app.sdl_context.event_pump().unwrap();

    let mut game_state = GameState::new();

    let mut last_frame_time: u32 = 0;
    while app.is_running {
        let ticks = app.timer.ticks();

        let delta_time = (ticks - last_frame_time) as f32 / 1000.0;
        last_frame_time = ticks;

        process_input(&mut event_pump, &mut game_state.player, &mut app.is_running);

        game_state.player.update(delta_time, &game_state.map);
        Ray::cast_all_rays(&mut game_state);

        render(&mut app, &mut game_state);
    }
}
