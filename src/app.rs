use crate::prelude::*;

pub struct App<'a> {
    pub sdl_context: Sdl,
    pub timer: TimerSubsystem,
    pub renderer: Canvas<Window>,
    pub display_buffer: Vec<u32>,
    pub texture_creator: TextureCreator<WindowContext>,
    pub display_buffer_texture: RefCell<Texture<'a>>,
    pub is_running: bool,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
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

        let texture_creator = renderer.texture_creator();

        let display_buffer_texture = texture_creator
            .create_texture(
                PixelFormatEnum::RGBA32,
                sdl2::render::TextureAccess::Streaming,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .unwrap();

        let display_buffer_texture =
            unsafe { std::mem::transmute::<_, Texture<'a>>(display_buffer_texture) };

        App {
            sdl_context,
            timer,
            renderer,
            display_buffer,
            texture_creator,
            display_buffer_texture: RefCell::new(display_buffer_texture),
            is_running: true,
        }
    }

    pub fn display_buffer_raw(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.display_buffer.as_ptr() as *const u8,
                self.display_buffer.len() * 4,
            )
        }
    }
}
