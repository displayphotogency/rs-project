use ggez::{Context, GameResult};

use sprite::Loader;
use sprite::animation::Animation;
use rendering::asset_storage::AssetStorage;

pub struct AnimationLoader;
impl AnimationLoader {
    pub fn load_assets(context: &mut Context, storage: &mut AssetStorage) -> GameResult<()> {
        let anim1 = Loader::load_sprite_sheet(context, "/animations/anim1")?;

        storage.animations.extend(vec![
            ("anim1", Animation::new(anim1)),
        ]);
        Ok(())
    }
}
