//! Rayst engine structs to do with the gameplay elements,
//! like the player, entities, et cetera. 

use std::f32::consts::PI;

use notan::{math::Vec3, prelude::Texture};

/// Entity Trait
pub trait Entity {
    /// Get entity position
    fn get_pos(&self) -> Vec3;

    /// Return angle the entity is facing (degrees)
    fn get_angle_deg(&self) -> f32;
    
    // I thought it would make sense to have the API have an automatically
    // implemented radians function too, seeing as it is often used in maths
    // that some developers may need (including internally in Rayst).
    /// Return angle the entity is facing (radians)
    fn get_angle_rad(&self) -> f32;

    /// Translate entity by a Vec3
    fn translate(&mut self, translation: Vec3);
}

/// Rayst engine Player structure. Stores information
/// relevant to the exact player.
pub struct Player {
    pos: Vec3,
    angle_deg: f32,
    pub weapon: Box<dyn Weapon>
}

impl Entity for Player {
    fn get_pos(&self) -> Vec3 {
        self.pos    
    }

    fn translate(&mut self, translation: Vec3) {
        self.pos = self.pos + translation;
    }

    fn get_angle_deg(&self) -> f32 {
        self.angle_deg
    }

    fn get_angle_rad(&self) -> f32 {
        (self.angle_deg / 360.0 ) * (2.0 * PI)
    }
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
}

