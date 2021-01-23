use rand::{
    Rng,
    SeedableRng,
    rngs::StdRng,
};
use crate::System;

pub struct SystemImpl {
    pub screen: [u8; 128 * 128],
    pub color: u8,
    pub std_rng: StdRng,
}

impl SystemImpl {
    pub fn new() -> SystemImpl {
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
