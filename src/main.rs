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

fn generate_projection(app: &mut App, game_state: &GameState) {
    let (player, rays) = (&game_state.player, &game_state.rays);

    for i in 0..NUM_RAYS {
        let perp_distance: f32 = rays[i as usize].distance
            * f32::cos(rays[i as usize].ray_angle - player.rotation_angle);

        let distance_proj_plane: f32 = (WINDOW_WIDTH as f32 / 2.0) / f32::tan(FOV_ANGLE / 2.0);
        let projected_wall_height = (TILE_SIZE as f32 / perp_distance) * distance_proj_plane;

        let wall_strip_height: i32 = projected_wall_height as i32;

        let mut wall_top_pixel: i32 = (WINDOW_HEIGHT as i32 / 2) - (wall_strip_height / 2);
        wall_top_pixel = if wall_top_pixel < 0 {
            0
        } else {
            wall_top_pixel
        };

        let mut wall_bottom_pixel: i32 = (WINDOW_HEIGHT as i32 / 2) + (wall_strip_height / 2);
        wall_bottom_pixel = if wall_bottom_pixel > WINDOW_HEIGHT as i32 {
            WINDOW_HEIGHT as i32
        } else {
            wall_bottom_pixel
        };

        for j in 0..wall_top_pixel {
            app.renderer
                .set_draw_color(Color::RGBA(0x33, 0x33, 0x33, 0xFF));
            app.renderer
                .draw_point(Point::new(i as i32, j as i32))
                .unwrap();
        }

        for j in wall_top_pixel..wall_bottom_pixel {
            let color = if rays[i as usize].was_hit_vertical {
                Color::RGBA(255, 255, 255, 255)
            } else {
                Color::RGBA(0xCC, 0xCC, 0xCC, 0xFF)
            };
            app.renderer.set_draw_color(color);
            app.renderer
                .draw_point(Point::new(i as i32, j as i32))
                .unwrap();
            // app.display_buffer[(WINDOW_WIDTH * j as u32 + i) as usize] = 0xFFFFFFFF;
        }

        for j in wall_bottom_pixel..WINDOW_HEIGHT as i32 {
            app.renderer
                .set_draw_color(Color::RGBA(0x77, 0x77, 0x77, 0xFF));
            app.renderer
                .draw_point(Point::new(i as i32, j as i32))
                .unwrap();
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

    generate_projection(app, game_state);
    // render_display_buffer(app);
    // clear_display_buffer(app);

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
