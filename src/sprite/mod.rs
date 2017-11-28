use ggez::{Context, GameError, GameResult};
use ggez::graphics::Image;
use serde_json;

use marker::SpriteData;

pub mod animation;

pub struct Loader;
impl Loader {
    pub fn load_sprite_sheet(context: &mut Context, name: &str) -> GameResult<MarkedTiles> {
        let data_file = context.filesystem.open(format!("{}.json", name)).or_else(|_| {
            context.filesystem.open(format!("{}-marked.json", name))
        })?;

        let data: Vec<SpriteData> = serde_json::from_reader(data_file).map_err(|_| {
            GameError::ResourceLoadError(format!("Json file is corrupt, check fields: {}", name))
        })?;
        let image = Image::new(context, format!("{}.png", name))?;

        Ok(MarkedTiles { data, image: image })
    }
}

pub struct MarkedTiles {
    pub data: Vec<SpriteData>,
    pub image: Image,
}
