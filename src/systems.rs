use ggez::Context;
use specs::{System, ReadStorage, Fetch, FetchMut, WriteStorage};

use asset_storage::AssetStorage;
use camera::Camera;
use components::{PositionComponent,
                 RenderComponent,
                 ChaseCameraComponent,
                 ScaleComponent,
                 DirectionComponent};

pub struct RenderSystem<'c> {
    context: &'c mut Context,
}

impl<'c> RenderSystem<'c> {
    pub fn new(context: &'c mut Context) -> RenderSystem<'c> {
        RenderSystem { context }
    }
}

impl<'a, 'c> System<'a> for RenderSystem<'c> {
    type SystemData = (Entities<'a>,
                       FetchMut<'a, AssetStorage>,
                       Fetch<'a, Camera>,
                       ReadStorage<'a, RenderComponent>,
                       ReadStorage<'a, PositionComponent>,
                       ReadStorage<'a, ScaleComponent>,
                       ReadStorage<'a, DirectionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
             mut assets,
             camera_comp,
             render_comp,
             position_comp,
             scale_comp,
             direction_comp) = data;
    }
}
