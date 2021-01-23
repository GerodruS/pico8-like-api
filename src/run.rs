use std::time::Duration;
use crate::{
    Game,
    system_impl::SystemImpl,
};

pub fn run(game: &mut impl Game) -> Result<(), String> {
    let mut system = SystemImpl::new()
        .expect("could not start the system");

    game.init(&mut system);

    while system.before_update() {
        game.update(&mut system);
        game.draw(&mut system);

        system.render()?;

        ::std::thread::sleep(Duration::new(0, 0));
    }

    Ok(())
}

