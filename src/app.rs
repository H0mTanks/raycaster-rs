use crate::prelude::*;

pub struct App {
    pub sdl_context: Sdl,
    pub timer: TimerSubsystem,
    pub renderer: Canvas<Window>,
    pub display_buffer: Vec<u32>,
    pub is_running: bool,
}

impl App {
    pub fn new() -> App {
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

        let display_buffer: Vec<u32> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];

        App {
            sdl_context,
            timer,
            renderer,
            display_buffer,
            is_running: true,
        }
    }
}
