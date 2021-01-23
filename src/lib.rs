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
    println!("Hello, world!");
    Ok(())
}