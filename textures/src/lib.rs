//! This is the library for texture operations as well
//! as the serialisation structures for the textures.

/// Constant that represents the missing texture path relative 
/// to this file. The missing texture is used to represent when a
/// texture is not found by the game engine.
pub const MISSING_TEX_PATH: &str = "./assets/missingtexture.png";

#[cfg(test)]
mod tests {
    use super::*;
}
