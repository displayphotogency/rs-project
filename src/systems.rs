use ggez::Context;
use ggez::graphics::{Vector2, Point2, DrawParam};
use specs::{System, ReadStorage, Fetch, FetchMut, WriteStorage};
use rayon::iter::ParallelIterator;

use std::collections::BTreeMap;

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
        let default_scale = ScaleComponent::new(1.0, 1.0);
        let mut layers = BTreeMap::new();

        for (e, r, pos) in (&*entities, &render_comp, &position_comp).join() {
            let mut scale: ScaleComponent = scale_comp.get(e).unwrap_or_else(|| &default_scale).clone();

            if let Some(&DirectionComponent::Left) = direction_comp.get(e) {
                scale.x = -scale.x;
            }

            layers.entry(r.layer)
                  .or_insert(vec![(r.render_type.clone(), pos.clone(), scale]))
                  .push((r.render_type.clone(), pos.clone(), scale));

            for (_, data) in layers.into_iter() {
                for (rt, pos, scale) in data.into_iter() {
                    match rt {
                        RenderableType::Animation { id, frame, length } => {
                            if let Some(ref mut batch) = assets.animations.get_mut(id) {
                                if frame < length {
                                    let frame = batch.frames[frame];
                                    let dp = DrawParam {
                                        dest: Point2::new(pos.x, pos.y)
                                    }
                                }
                            }
                        }
                        RenderableType::Image { id } => if let Some(i) = assets.images.get(id) {

                        }
                        RenderableType::Batch { id } => if let Some(b) = assets.batches.get(id) {

                        }
                    }
                }
            }
        }
    }
}
