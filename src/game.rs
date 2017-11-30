use ggez::{Context, GameResult};
use ggez::event::{EventHandler, Keycode, MouseButton, MouseState, Axis,
                  Button, Mod};
use specs::{World, Dispatcher, DispatcherBuilder};

use components::{register_components, Position, Renderable, ChaseCamera, RenderableType};
use rendering::asset_storage::AssetStorage;
use map::Map;
use animation::loader::AnimationLoader;
use systems{RenderSystem, ChaseCameraSystem, PositionSystem};

pub struct Game<'a, 'b> {
    pub world: World,
    pub player_count: usize,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl <'a, 'b> Game<'a, 'b> {
    pub fn new(context: &mut Context) -> GameResult<Game<'a, 'b>> {
        let mut world = World::new();
        let mut pc = 0;

        register_components(&mut world);

        let mut asset_storage = AssetStorage::empty();
        // let map = Map::load(context)?;
        // let RenderableMap {
        //     background,
        //     ground_batch,
        //     objects_batch,
        //     terrain,
        // } = RenderableMap::build(map);
        // asset_storage.images.insert("map-background", background);
        // asset_storage.batches.insert("map-ground", ground_batch);
        // asset_storage.batches.insert("map-objects", objects_batch);
        // world.add_resource(MapTerrain { terrain });
        AnimationLoader::load_assets(context, &mut asset_storage)?;
        world.add_resource::<AssetStorage>(asset_storage);

        world.create_entity().with(PositionComponent::new(0.0, 0.0))
                             .with(RenderComponent {
                                 layer: 1,
                                 render_type: RenderComponentType::Batch { id: "map-background" },
                             })
                             .with(ChaseCameraComponent)
                             .build();

        // world.add_resource(MousePointer(0.0, 0.0));
        // world.add_resource(DeltaTime { delta: 0.0 });
        // world.add_resource(PlayerInput::new());

        let (w, h) = (context.conf.window_mode.width, context.conf.window_mode.height);
        let hc = h as f64 / w as f64;
        let fov = w as f64 * 1.5;

        world.add_resource(Camera::new(w, h, fov, hc * fov));
        // Player::spawn(&mut world, Vector2::new(500.0, 500.0), true, true, &mut pc);

        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .add(systems::RenderSystem)
            .add(systems::ChaseCameraSystem)
            .add(systems::PositionSystem)
            .build();

        Ok(Game {
            world,
            player_count: pc,
            dispatcher,
        })
    }
}

impl<'a, 'b> EventHandler for Game<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // if timer::get_ticks(ctx) % 100 == 0 {
        //     println!("FPS: {}", timer::get_fps(ctx));
        // }

        // self.world.write_resource::<DeltaTime>().delta = seconds(&dt);

        // if timer::check_update_time(ctx, 30) {
        //     PlayerFixedUpdateSystem.run_now(&mut self.world.res);
        //     AnimationFFSystem.run_now(&mut self.world.res);
        // }

        // self.dispatcher.dispatch(&mut self.world.res);
        // self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // graphics::clear(ctx);

        // {
        //     let mut rs = RenderingSystem::new(ctx);
        //     rs.run_now(&mut self.world.res);
        // }

        // graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, context: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        // let mut input = self.world.write_resource::<PlayerInput>();

        // if !repeat {
        //     match keycode {
        //         Keycode::Left => input.left = true,
        //         Keycode::Right => input.right = true,
        //         Keycode::Up => input.up = true,
        //         Keycode::Down => input.down = true,
        //         Keycode::LCtrl => input.slide = true,
        //         Keycode::Space => input.jump = true,
        //         Keycode::LShift => input.attack = true,
        //         _ => (),
        //     }
        // }
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        //let mut input = self.world.write_resource::<PlayerInput>();
        //if !repeat {
        //    //wat?
        //    match keycode {
        //        Keycode::Left => input.left = false,
        //        Keycode::Right => input.right = false,
        //        Keycode::Up => input.up = false,
        //        Keycode::Down => input.down = false,
        //        _ => (),
        //    }
        //}
    }

    fn controller_button_down_event(&mut self, context: &mut Context, btn: Button, _instance_id: i32) {
        // let mut input = self.world.write_resource::<PlayerInput>();
        // match btn {
        //     Button::A => input.jump = true,
        //     Button::X => input.attack = true,
        //     Button::B => input.slide = true,
        //     // Button::LeftShoulder => self.player.mv.position = Vector2::new(300.0, 500.0),
        //     _ => (),
        // }
    }
    fn controller_button_up_event(&mut self, context: &mut Context, _btn: Button, _instance_id: i32) {}
    fn controller_axis_event(&mut self, context: &mut Context, axis: Axis, value: i16, _instance_id: i32) {
        // let mut input = self.world.write_resource::<PlayerInput>();
        // match axis {
        //     Axis::LeftX => {
        //         if value > 7500 {
        //             input.right = true
        //         } else {
        //             input.right = false
        //         };
        //         if value < -7500 {
        //             input.left = true
        //         } else {
        //             input.left = false
        //         }
        //     }
        //     Axis::LeftY => {
        //         if value > 7500 {
        //             input.down = true
        //         } else {
        //             input.down = false
        //         }
        //     }
        //     _ => (),
        // }
    }

    fn mouse_button_down_event(&mut self, context: &mut Context, button: MouseButton, x: i32, y: i32) {
        // if button == event::MouseButton::Left {
        //     let p = self.world.read_resource::<Camera>().screen_to_world_coords((x, y));
        //     Player::spawn(&mut self.world, p, false, false, &mut self.player_count)
        // }
    }

    fn mouse_motion_event(&mut self, context: &mut Context, _state: MouseState, x: i32, y: i32, _: i32, _: i32) {
        // let coords = self.world.read_resource::<Camera>().screen_to_world_coords((x, y));
        // let mut mp = self.world.write_resource::<MousePointer>();
        // mp.0 = coords.x;
        // mp.1 = coords.y;
    }

    fn mouse_wheel_event(&mut self, context: &mut Context, _: i32, _: i32) {
        // let mp = self.world.read_resource::<MousePointer>().clone();
        // let p = Vector2::new(mp.0, mp.1);
        // Player::spawn(&mut self.world, p, false, false, &mut self.player_count);
    }
}
