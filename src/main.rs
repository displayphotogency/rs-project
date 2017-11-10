extern crate ggez;
extern crate rand;
extern crate specs;

// use ggez::audio;
use ggez::conf;
// use ggez::event::*;
use ggez::Context;
// use ggez::{Context, GameResult};
use ggez::graphics;
// use ggez::timer;

use specs::{Component, Join, DispatcherBuilder, ReadStorage, System, VecStorage,
            World, WriteStorage, RunNow};

// use std::env;
// use std::path;

// specs stuff
#[derive(Debug)]
struct VelocityComp(f32, f32);
impl Component for VelocityComp {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct PositionComp {
    point: graphics::Point
}
impl Component for PositionComp {
    type Storage = VecStorage<Self>;
}

struct PositionSys;
impl<'a> System<'a> for PositionSys {
    type SystemData = (WriteStorage<'a, PositionComp>, ReadStorage<'a, VelocityComp>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            println!("run PositionSys");
            pos.point.x += vel.0;
            pos.point.y += vel.1;
        }
    }
}

#[derive(Debug)]
struct RenderComp {
    image: graphics::Image,
}
impl Component for RenderComp {
    type Storage = VecStorage<Self>;
}

struct RenderSys;
impl<'a> System<'a> for RenderSys {
    type SystemData = (ReadStorage<'a, RenderComp>, ReadStorage<'a, PositionComp>,
                       ReadStorage<'a, ContextComp>, WriteStorage<'a, ContextComp>);

    fn run(&mut self, (render, pos, context): Self::SystemData) {
        for (render, pos) in (&render, &pos).join() {
            println!("run RenderSys");
            graphics::draw(context, &render.image, pos.point, 0.0);
        }
    }
}

#[derive(Debug)]
struct ContextComp {
    context: Context,
}
impl Component for ContextComp {
    type Storage = VecStorage<Self>;
}


fn main() {
    let mut conf = conf::Conf::new();
    conf.window_title = "".to_string();
    conf.window_height = 640;
    conf.window_height = 480;

    let context = &mut Context::load_from_conf("", "", conf).unwrap();

    let mut world = World::new();
    world.register::<PositionComp>();
    world.register::<VelocityComp>();
    world.register::<RenderComp>();

    world.create_entity()
        .with(VelocityComp(2.0, 0.0))
        .with(PositionComp { point: graphics::Point { x: 0.0, y: 0.0 } })
        .with(RenderComp { image: graphics::Image::new(context, "resources/test.png").unwrap() })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .add(PositionSys, "position", &[])
        .build();

    // Loop
    // for multithread-able systems
    dispatcher.dispatch(&world.res);

    // Can't use dispatch because we need to pass `context`, which is not
    // thread-safe
    RenderSys.run_now(&world.res);
}
