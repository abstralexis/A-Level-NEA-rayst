//! Rayst engine structs to do with the gameplay elements,
//! like the player, entities, et cetera. 

use notan::{math::Vec3, prelude::Texture};

/// Rayst engine Player structure. Stores information
/// relevant to the exact player.
pub struct Player {
    pub pos: Vec3,
    pub angle: f32,
    pub weapon: Box<dyn Weapon>
}

/// Enum used for dictating whether a weapon is
/// single fire or rapid fire
pub enum FireMode {
    Semi,
    Auto,
}

/// Attack type structure
pub struct Attack {
    damage: u32,
    range_dropoff: dyn Fn(u32) -> u32, // Damage dropoff function 
}

/// Animation type alias
pub type Animation = Vec<Texture>;

/// Weapon trait for Rayst Engine
pub trait Weapon {
    /// Looping animation for when idle
    fn idle_animation(&self) -> Animation;
    /// Looping animation for when walking
    fn walk_animation(&self) -> Animation;
    /// Animation for when reloading. Reload lasts as long as animation.
    fn reload_animation(&self) -> Animation;
    /// Animation for when firing. Loops if `FireMode` is set to `Auto`, else 
    /// the firing speed is dictated by the length of the animation.
    fn fire_animation(&self) -> Animation;
    /// Gets the relevant attack structure
    fn get_attack(&self) -> Attack;
    /// Adjust player position based off of a vector translation
    fn translate(&mut self, direction: &Vec3);
}