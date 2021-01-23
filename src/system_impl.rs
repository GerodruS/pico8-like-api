use rand::{
    Rng,
    SeedableRng,
    rngs::StdRng,
};
use sdl2::{
    EventPump,
    event::Event,
    render::WindowCanvas,
    pixels::Color,
    rect::Point,
};
use crate::System;

pub struct SystemImpl {
    pub event_pump: EventPump,
    pub window_canvas: WindowCanvas,
    pub screen: [u8; 128 * 128],
    pub color: u8,
    pub std_rng: StdRng,
    palette: [Color; 16],
}

impl SystemImpl {
    pub fn new() -> Result<SystemImpl, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("game tutorial", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .expect("could not initialize video subsystem");

        // TODO:
        // sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0");

        let mut window_canvas = window.into_canvas().build()
            .expect("could not make a canvas");
        window_canvas.set_logical_size(128, 128)
            .expect("set_logical_size");


        let event_pump = sdl_context.event_pump()?;

        let palette = [
            Color::RGB(0, 0, 0),
            Color::RGB(29, 43, 83),
            Color::RGB(126, 37, 83),
            Color::RGB(0, 135, 81),

            Color::RGB(171, 82, 54),
            Color::RGB(95, 87, 79),
            Color::RGB(194, 195, 199),
            Color::RGB(255, 241, 232),

            Color::RGB(255, 0, 77),
            Color::RGB(255, 163, 0),
            Color::RGB(255, 236, 39),
            Color::RGB(0, 228, 54),

            Color::RGB(41, 173, 255),
            Color::RGB(131, 118, 156),
            Color::RGB(255, 119, 168),
            Color::RGB(255, 204, 170),
        ];

        Ok(SystemImpl {
            event_pump,
            window_canvas,
            screen: [0; 128 * 128],
            color: 6,
            std_rng: StdRng::from_entropy(),
            palette,
        })
    }

    pub fn before_update(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    return false
                },
                _ => {}
            }
        }

        true
    }

    pub fn render(&mut self) -> Result<(), String> {
        self.window_canvas.set_draw_color(Color::BLACK);
        self.window_canvas.clear();

        // TODO: consider to use separated canvas
        for i in 0..self.screen.len() {
            let color_index = self.screen[i];
            let color = self.palette[color_index as usize];
            self.window_canvas.set_draw_color(color);
            self.window_canvas.draw_point(Point::new((i % 128) as i32, (i / 128) as i32))?;
        }

        self.window_canvas.present();

        Ok(())
    }
}

impl System for SystemImpl {
    fn pget(&mut self, x: u8, y: u8) -> u8 {
        self.screen[(x + y * 128) as usize]
    }

    fn pset(&mut self, x: u8, y: u8, color: u8) {
        self.color(color);
        if x < 128 && y < 128 {
            let (x, y) = (x as usize, y as usize);
            self.screen[(x + y * 128) as usize] = color;
        }
    }

    fn color(&mut self, color: u8) {
        self.color = color;
    }

    fn cls(&mut self, color: u8) {
        for p in self.screen.iter_mut() {
            *p = color;
        }
    }
    //
    // fn circ(&mut self, x: u8, y: u8, radius: u8, color: u8) {
    //     unimplemented!()
    // }
    //
    // fn circfill(&mut self, x: u8, y: u8, radius: u8, color: u8) {
    //     unimplemented!()
    // }

    fn line(&mut self, x0: u8, y0: u8, x1: u8, y1: u8, color: u8) {

        if y0 == y1 {
            let (x0, x1) = (x0.min(x1), x0.max(x1));
            for x in x0..=x1 {
                self.pset(x, y0, color);
            }
        } else if x0 == x1 {
            let (y0, y1) = (y0.min(y1), y0.max(y1));
            for y in y0..=y1 {
                self.pset(x0, y, color);
            }
        } else {
            let dx = (x1 as i32 - x0 as i32).abs();
            let sx: i32 = if x0 < x1 { 1 } else { -1 };
            let dy = -(y1 as i32 - y0 as i32).abs();
            let sy: i32 = if y0 < y1 { 1 } else { -1 };
            let mut err = dx + dy;

            let mut x0 = x0 as i32;
            let mut y0 = y0 as i32;

            loop {
                self.pset(x0 as u8, y0 as u8, color);

                if x0 == x1 as i32 && y0 == y1 as i32 {
                    break
                }

                let err2 = 2 * err;

                if err2 >= dy
                {
                    err += dy;
                    x0 += sx;
                }

                if err2 <= dx
                {
                    err += dx;
                    y0 += sy;
                }
            }
        }

        // // TODO: shouldn't be updated when invoked from other primitives, eg rect
        // _state.lastLineEnd.x = x1;
        // _state.lastLineEnd.x = y1;
    }

    fn rnd(&mut self, x: f32) -> f32 {
        // TODO: check inclusive/exclusive
        self.std_rng.gen_range(0.0..x)
    }

    fn srand(&mut self, x: u64) {
        self.std_rng = StdRng::seed_from_u64(x);
    }

    fn cos(&self, x: f32) -> f32 {
        (x * 2.0 * std::f32::consts::PI).cos()
    }

    fn sin(&self, x: f32) -> f32 {
        (-x * 2.0 * std::f32::consts::PI).sin()
    }
}
