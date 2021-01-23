use crate::System;

pub trait Game {
    fn init(&mut self, _sys: &mut impl System) {}
    fn update(&mut self, _sys: &mut impl System) {}
    fn draw(&mut self, _sys: &mut impl System) {}
}
