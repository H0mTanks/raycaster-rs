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

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x < WINDOW_WIDTH as i32 && y < WINDOW_HEIGHT as i32 && x >= 0 && y >= 0 {
            self.display_buffer[(WINDOW_WIDTH as i32 * y + x) as usize] = color;
        }
    }

    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: u32) {
        let dx: i32 = i32::abs(x1 - x0);
        let sx: i32 = if x0 < x1 { 1 } else { -1 };

        let dy: i32 = -i32::abs(y1 - y0);
        let sy: i32 = if y0 < y1 { 1 } else { -1 };

        let mut error: i32 = dx + dy;
        loop {
            self.draw_pixel(x0, y0, color);
            if x0 == x1 && y0 == y1 {
                break;
            }

            let error2: i32 = error * 2;
            if error2 >= dy {
                error += dy;
                x0 += sx;
            }

            if error2 <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        for j in y..y + height {
            for i in x..x + width {
                self.draw_pixel(i, j, color);
            }
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
