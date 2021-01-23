use sdl2::{
    rect::Point,
    event::Event,
    pixels::Color,
    render::WindowCanvas,
};
use std::time::Duration;
use crate::{
    Game,
    system_impl::SystemImpl,
};

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
        canvas.draw_point(Point::new((i % 128) as i32, (i / 128) as i32))?;
    }

    canvas.present();

    Ok(())
}
