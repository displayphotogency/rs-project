extern crate ggez;
extern crate nalgebra;
extern crate rand;
extern crate rayon;
extern crate serde_json;
extern crate specs;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate specs_derive;

use ggez::{Context, event, graphics};
use ggez::conf::{Backend, Conf, WindowMode, WindowSetup, FullscreenType, NumSamples};

mod game;
mod components;
mod rendering;
mod sprite;
mod marker;
mod map;
mod animation;

fn main() {
    let default_conf = Conf {
        window_title: String::from("Testing"),
        window_icon: String::from("/icon.png"),
        window_mode: WindowMode {
            width: 800,
            height: 600,
            borderless: false,
            fullscreen_type: FullscreenType::Off,
            vsync: true,
            min_width: 0,
            max_width: 0,
            min_height: 0,
            max_height: 0,
        },
        window_setup: WindowSetup {
            resizable: false,
            allow_highdpi: true,
            samples: NumSamples::One,
        },
        backend: Backend::OpenGL { major: 3, minor: 0 },
    };

    let context = &mut Context::load_from_conf("gm.conf", "author", default_conf).unwrap();
    graphics::set_default_filter(context, graphics::FilterMode::Nearest);

    let mut state = game::Game::new(context).unwrap();
    event::run(context, &mut state).unwrap();
}
