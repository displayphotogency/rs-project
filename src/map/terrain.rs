use ggez::graphics::Vector2;

use nalgebra;

pub struct Terrain {
    pub terrain: Vec<Vec<TileType>>,
    pub position: Vector2,
    pub width: usize,
    pub height: usize,
    pub tile_size: f32,
}

impl Terrain {
    pub fn get_tile_at_point(&self, point: Vector2) -> nalgebra::Vector2<isize> {
        nalgebra::Vector2::new(
            ((point.x - self.position.x as f32 + self.tile_size / 2.0) / self.tile_size) as isize,
            ((point.y - self.position.y as f32 + self.tile_size / 2.0) / self.tile_size) as isize,
        )
    }

    pub fn get_tile_y_at_point(&self, y: f32) -> isize {
        ((y - self.position.y + self.tile_size / 2.0) / self.tile_size) as isize
    }

    pub fn get_tile_x_at_point(&self, x: f32) -> isize {
        ((x - self.position.x + self.tile_size / 2.0) / self.tile_size) as isize
    }

    pub fn get_map_tile_position(&self, x: isize, y: isize) -> Vector2 {
        Vector2::new(
            x as f32 * self.tile_size + self.position.x,
            y as f32 * self.tile_size + self.position.y,
        )
    }

    pub fn get_map_tile_position_vec(&self, coords: nalgebra::Vector2<isize>) -> Vector2 {
        self.get_map_tile_position(coords.x, coords.y)
    }

    #[inline]
    fn in_bounds(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }

    pub fn get_tile(&self, x: isize, y: isize) -> TileType {
        if let Some((x, y)) = self.in_bounds(x, y) {
            self.terrain[y][x]
        } else {
            TileType::Block
        }
    }

    pub fn is_obstacle(&self, x: isize, y: isize) -> bool {
        self.get_tile(x, y) == TileType::Block
    }

    pub fn is_empty(&self, x: isize, y: isize) -> bool {
        if let Some((x, y)) = self.in_bounds(x, y) {
            self.terrain[y][x] == TileType::Empty
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Empty,
    Block,
}
