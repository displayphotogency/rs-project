extern crate ggez;
extern crate nalgebra as na;
extern crate rand;
extern crate rayon;
extern crate serde_json;
extern crate specs;

use ggez::{Context, event, graphics};
use ggez::conf::Conf;

mod game;
mod components;

fn main() {
    let default_conf = Conf {
        window_height: 480,
        window_width: 640,
        window_title: String::from("Testing"),
        window_icon: String::from("icon.png"),
        vsync: false,
        resizable: true,
    };

    let context = &mut Context::load_from_conf("gm.conf", "author", default_conf).unwrap();
    graphics::set_default_filter(context, graphics::FilterMode::Nearest);

    let mut state = game::Game::new(context).unwrap();
    event::run(context, &mut state).unwrap();
}
