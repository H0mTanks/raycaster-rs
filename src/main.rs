#[macro_use]
extern crate lazy_static;
extern crate atomic_float;

mod prelude {
    use atomic_float::AtomicF32;
    use std::sync::atomic::AtomicU32;

    pub const WINDOW_WIDTH: u32 = 800;
    pub const WINDOW_HEIGHT: u32 = 600;

    pub static DELTA_TIME: AtomicF32 = AtomicF32::new(0.0);
    pub static LAST_FRAME_TIME: AtomicU32 = AtomicU32::new(0);
}

use core::sync::atomic::Ordering;
use prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
pub use sdl2::Sdl;
use sdl2::{EventPump, TimerSubsystem};

fn sdl_init() -> (Sdl, TimerSubsystem, Canvas<Window>, bool) {
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Raycaster", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let timer = sdl_context.timer().unwrap();

    let mut renderer = window.into_canvas().software().build().unwrap();
    renderer.set_blend_mode(sdl2::render::BlendMode::Blend);

    (sdl_context, timer, renderer, true)
}

fn process_input(event_pump: &mut EventPump, running: &mut bool) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => *running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => *running = false,
            _ => {}
        }
    }
}

fn render(renderer: &mut Canvas<Window>) {
    renderer.set_draw_color(Color::RGBA(0, 0, 0, 255));
    renderer.clear();

    renderer.set_draw_color(Color::RGBA(155, 0, 0, 255));
    let rect: Rect = Rect::new(0, 0, 20, 20);
    // renderer.draw_rect(rect).unwrap();
    renderer.fill_rect(rect).unwrap();

    renderer.present();
}

fn main() {
    let (sdl_context, timer, mut renderer, mut running) = sdl_init();
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        let ticks = timer.ticks();

        DELTA_TIME.store(
            (ticks - LAST_FRAME_TIME.load(Ordering::SeqCst)) as f32 / 1000.0,
            Ordering::SeqCst,
        );

        LAST_FRAME_TIME.store(ticks, Ordering::SeqCst);

        println!(
            "{} {} {}",
            DELTA_TIME.load(Ordering::SeqCst),
            LAST_FRAME_TIME.load(Ordering::SeqCst),
            ticks
        );
        process_input(&mut event_pump, &mut running);
        render(&mut renderer);
    }
}
