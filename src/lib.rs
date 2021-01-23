use sdl2::image;
use sdl2::image::InitFlag;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use rand_distr::{Normal, Distribution};
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use sdl2::render::BlendMode;
use std::time::Duration;

pub struct SystemImpl {
    screen: [u8; 128 * 128],
    color: u8,
    std_rng: StdRng,
}

impl SystemImpl {
    fn new() -> SystemImpl {
        SystemImpl {
            screen: [0; 128 * 128],
            color: 6,
            std_rng: StdRng::from_entropy(),
        }
    }
}

impl System for SystemImpl {
    fn pget(&mut self, x: u8, y: u8) -> u8 {
        self.screen[(x + y * 128) as usize]
    }

    fn pset(&mut self, x: u8, y: u8, color: u8) {
        self.color(color);
        if 0 <= x && x < 128 && 0 <= y && y < 128 {
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

    fn circ(&mut self, x: u8, y: u8, radius: u8, color: u8) {
        unimplemented!()
    }

    fn circfill(&mut self, x: u8, y: u8, radius: u8, color: u8) {
        unimplemented!()
    }

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

pub trait System {
    // Get or set the color of a pixel at x, y.
    fn pget(&mut self, x: u8, y: u8) -> u8;
    fn pset(&mut self, x: u8, y: u8, color: u8);

    // Set the current colour to be used by drawing functions
    fn color(&mut self, color: u8);

    // Clear the screen and reset the clipping rectangle
    fn cls(&mut self, color: u8);

    // Draw a circle or filled circle at x,y with radius r
    // If r is negative, the circle is not drawn
    fn circ(&mut self, x: u8, y: u8, radius: u8, color: u8);
    fn circfill(&mut self, x: u8, y: u8, radius: u8, color: u8);
    //
    // line x0 y0 [x1 y1] [col]
    //
    // draw line
    // if x1,y1 are not given the end of the last drawn line is used
    fn line(&mut self, x0: u8, y0: u8, x1: u8, y1: u8, color: u8);
    //
    //
    // rect     x0 y0 x1 y1 [col]
    // rectfill x0 y0 x1 y1 [col]
    //
    // Draw a rectangle or filled rectangle

    // Returns a random number n, where 0 <= n < x
    fn rnd(&mut self, x: f32) -> f32;

    // Sets the random number seed
    // The seed is automatically randomized on cart startup
    fn srand(&mut self, x: u64);

    // Returns the cosine of x, where 1.0 indicates a full circle
    // sin is inverted to suit screenspace
    // e.g. sin(0.25) returns -1
    //
    // If you'd prefer radian-based trig functions without the y inversion,
    // use: x.cos() / x.sin()
    fn cos(&self, x: f32) -> f32;
    fn sin(&self, x: f32) -> f32;
}

pub trait Game {
    fn init(&mut self, sys: &mut impl System) {}
    fn update(&mut self, sys: &mut impl System) {}
    fn draw(&mut self, sys: &mut impl System) {}
}

pub fn run(game: &mut impl Game) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");

    // TODO:
    // sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    canvas.set_logical_size(128, 128)
        .expect("set_logical_size");

    let mut system = SystemImpl::new();
    game.init(&mut system);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }

        game.update(&mut system);
        game.draw(&mut system);
        render(&mut canvas, &system)?;

        ::std::thread::sleep(Duration::new(0, 0));
    }

    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    system: &SystemImpl,
) -> Result<(), String> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    let colors = [
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

    for i in 0..system.screen.len() {
        let color_index = system.screen[i];
        let color = colors[color_index as usize];
        canvas.set_draw_color(color);
        canvas.draw_point(Point::new((i % 128) as i32, (i / 128) as i32));
    }

    canvas.present();

    Ok(())
}