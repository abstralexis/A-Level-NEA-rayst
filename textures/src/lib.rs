//! This is the library for texture operations as well
//! as the serialisation structures for the textures.

/// Constant that represents the missing texture path relative
/// to this file. The missing texture is used to represent when a
/// texture is not found by the game engine.
pub const MISSING_TEX_PATH: &str = "./assets/missingtexture.png";

use notan::app::Graphics;
use notan::prelude::{Texture, TextureBuilder};
use std::collections::HashMap;
use std::fs::read;

#[allow(dead_code)]
pub struct TextureLoader {
    textures: HashMap<String, Texture>,
}
impl TextureLoader {
    pub fn new(gfx: &mut Graphics) -> Self {
        let missingtex = TextureBuilder::new(gfx)
            .from_image(
                read(MISSING_TEX_PATH)
                    .expect("Texture Loading Failed")
                    .as_slice(),
            )
            .build()
            .expect("Texture Building Failed");

        let mut textures = HashMap::new();
        textures.insert("missingtex".to_owned(), missingtex);

        TextureLoader { textures }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notan::{draw::DrawConfig, prelude::*};

    #[test]
    fn panic_test() {
        notan::init()
            .add_config(DrawConfig)
            .draw(draw)
            .build()
            .unwrap();

        fn draw(_app: &mut App, gfx: &mut Graphics) {
            TextureLoader::new(gfx);
        }
    }
}
