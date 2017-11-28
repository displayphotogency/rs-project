use ggez::{Context, GameResult};
use ggez::graphics::Image;

use sprite::{Loader, MarkedTiles};

pub struct LoadedAssets {
    pub ground: MarkedTiles,
    pub objects: MarkedTiles,
    pub background: Image,
}

impl LoadedAssets {
    pub fn load_assets(context: &mut Context) -> GameResult<LoadedAssets> {
        let gnd = Loader::load_sprite_sheet(context, "/map/ground")?;
        let obj = Loader::load_sprite_sheet(context, "/map/objects")?;
        let bg = Image::new(context, "/map/background.png")?;

        Ok(LoadedAssets {
            ground: gnd,
            objects: obj,
            background: bg
        })
    }
}
