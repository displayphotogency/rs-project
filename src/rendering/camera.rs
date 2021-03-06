use ggez::{Context, GameResult};
use ggez::graphics::{Point2, Vector2, DrawParam, Drawable};

use nalgebra;

pub struct Camera {
    screen_size: Vector2,
    view_size: Vector2,
    view_center: Vector2,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32, view_width: f32, view_height: f32) -> Self {
        let screen_size = Vector2::new(screen_width as f32, screen_height as f32);
        let view_size = Vector2::new(view_width as f32, view_height as f32);
        Camera {
            screen_size: screen_size,
            view_size: view_size,
            view_center: nalgebra::zero(),
        }
    }

    pub fn move_by(&mut self, by: Vector2) {
        self.view_center += by;
    }

    pub fn move_to(&mut self, to: Vector2) {
        self.view_center = to;
    }

    pub fn draw_scale(&self) -> Point2 {
        Point2::new(
            (self.screen_size.x / self.view_size.x) as f32,
            (self.screen_size.y / self.view_size.y) as f32,
        )
    }

    pub fn world_to_screen_coords(&self, from: Vector2) -> (i32, i32) {
        let pixels_per_unit = self.screen_size.component_div(&self.view_size);
        let view_offset = from - self.view_center;
        let view_scale = view_offset.component_mul(&pixels_per_unit);


        let x = (*view_scale).x + (*self.screen_size).x / 2.0;
        let y = (*self.screen_size).y - ((*view_scale).y + (*self.screen_size).y / 2.0);
        (x as i32, y as i32)
    }

    pub fn screen_to_world_coords(&self, from: (i32, i32)) -> Vector2 {
        let (sx, sy) = from;
        let sx = sx as f32;
        let sy = sy as f32;
        let flipped_x = sx - ((*self.screen_size).x / 2.0);
        let flipped_y = -sy + (*self.screen_size).y / 2.0;
        let screen_coords = Vector2::new(flipped_x, flipped_y);
        let units_per_pixel = self.view_size.component_div(&self.screen_size);
        let view_scale = screen_coords.component_mul(&units_per_pixel);
        let view_offset = self.view_center + view_scale;

        view_offset
    }

    pub fn location(&self) -> Vector2 {
        self.view_center
    }

    pub fn size(&self) -> Vector2 {
        self.view_size
    }

    pub fn calculate_dest_point(&self, location: Vector2) -> Point2 {
        let (sx, sy) = self.world_to_screen_coords(location);
        Point2::new(sx as f32, sy as f32)
    }
}

pub trait CameraDraw
where
    Self: Drawable, {
    fn draw_ex_camera(
        &self,
        camera: &Camera,
        ctx: &mut Context,
        p: DrawParam,
    ) -> GameResult<()> {
        let dest = Vector2::new(p.dest.x as f32, p.dest.y as f32);
        let dest = camera.calculate_dest_point(dest);
        let scale = camera.draw_scale();
        let orig_scale = p.scale.clone();
        let mut my_p = p;
        my_p.dest = dest;
        my_p.scale = Point2::new(orig_scale.x * scale.x, orig_scale.y * scale.y);
        self.draw_ex(ctx, my_p)
    }

    fn draw_camera(
        &self,
        camera: &Camera,
        ctx: &mut Context,
        dest: Point2,
        rotation: f32,
    ) -> GameResult<()> {
        let dest = Vector2::new(dest.x as f32, dest.y as f32);
        let dest = camera.calculate_dest_point(dest);
        let scale = camera.draw_scale();
        self.draw_ex(
            ctx,
            DrawParam {
                dest,
                scale,
                rotation,
                ..Default::default()
            },
        )
    }
}


impl<T> CameraDraw for T
where
    T: Drawable,
{
}
