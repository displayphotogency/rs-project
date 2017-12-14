use ggez::Context;
use ggez::graphics::{Vector2, Point2, DrawParam, Drawable};
use specs::{Entities, System, ReadStorage, Fetch, FetchMut, WriteStorage, Join};
use rayon::iter::ParallelIterator;

use std::collections::BTreeMap;

use rendering::asset_storage::AssetStorage;
use rendering::camera::{Camera, CameraDraw};
use components::{PositionComponent,
                 RenderComponent,
                 RenderComponentType,
                 ChaseCameraComponent,
                 ScaleComponent,
                 MovingObjectComponent,
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
                  .or_insert(vec![(r.render_type.clone(), pos.clone(), scale)])
                  .push((r.render_type.clone(), pos.clone(), scale));
        }

        for (_, data) in layers.into_iter() {
            for (rt, pos, scale) in data.into_iter() {
                match rt {
                    RenderComponentType::Animation { id, frame, length } => {
                        if let Some(ref mut batch) = assets.animations.get_mut(id) {
                            if frame < length {
                                let frame = batch.frames[frame];
                                let dp = DrawParam {
                                    dest: Point2::new(pos.x, pos.y),
                                    src: frame,
                                    scale: Point2::new(scale.x, scale.y),
                                    ..Default::default()
                                };

                                let dest = Vector2::new(dp.dest.x as f32, dp.dest.y as f32);
                                let dest = camera_comp.calculate_dest_point(dest);
                                let scale = camera_comp.draw_scale();
                                let orig_scale = dp.scale.clone();
                                let mut my_p = dp;
                                my_p.dest = dest;
                                my_p.scale =
                                    Point2::new(orig_scale.x * scale.x, orig_scale.y * scale.y);
                                batch.batch.add(my_p);
                            }
                        }
                    }
                    RenderComponentType::Image { id } => if let Some(i) = assets.images.get(id) {
                        i.draw_ex_camera(
                            &*camera_comp,
                            self.context,
                            DrawParam {
                                dest: Point2::new(pos.x, pos.y),
                                scale: Point2::new(scale.x, scale.y),
                                ..Default::default()
                            },
                        ).unwrap();
                    },
                    RenderComponentType::Batch { id } => if let Some(b) = assets.batches.get(id) {
                        b.draw_ex_camera(
                            &*camera_comp,
                            self.context,
                            DrawParam {
                                dest: Point2::new(pos.x, pos.y),
                                scale: Point2::new(scale.x, scale.y),
                                ..Default::default()
                            },
                        ).unwrap();
                    },
                }
            }
        }

        for (_, batch) in assets.animations.iter_mut() {
            batch.batch
                 .draw_ex(self.context,
                          DrawParam {
                              dest: Point2::new(0.0, 0.0),
                              scale: Point2::new(1.0, 1.0),
                              ..Default::default()
                          },).unwrap();
            batch.batch.clear();
        }
    }
}

pub struct ChaseCameraSystem;
impl<'a> System<'a> for ChaseCameraSystem {
    type SystemData = (Fetch<'a, Camera>,
                       ReadStorage<'a, ChaseCameraComponent>,
                       WriteStorage<'a, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (cam, chase, mut pos) = data;

        for (pos, _) in (&mut pos, &chase).join() {
            let loc = cam.location();
            pos.x = loc.x as f32;
            pos.y = loc.y as f32;
        }
    }
}

pub struct PositionSystem;
impl<'a> System<'a> for PositionSystem {
    type SystemData = (ReadStorage<'a, MovingObjectComponent>,
                       WriteStorage<'a, PositionComponent>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mv, mut pos) = data;
        (&mv, &mut pos).join().for_each(|(mv, pos)| {
            pos.x = mv.position.x as f32;
            pos.y = mv.position.y as f32;
        });
    }
}
