//! The file containing entity structs, traits,
//! and example entities.
use crate::weapons::*;
use crate::animation::*;

use std::f32::consts::PI;

use notan::math::{Vec3};

/// Entity Trait
pub trait NonPlayerEntity {
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

    /// Movement behaviour
    fn do_movement(&mut self);

    /// Attack
    fn attack(&mut self);

    /// Is attacking?
    fn is_attacking(&self) -> bool;
}

/// Rayst engine Player structure. Stores information
/// relevant to the exact player.
pub struct Player {
    pos: Vec3,
    angle_deg: f32,
    pub weapon: Box<dyn Weapon>
}

impl Player {
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
        self.angle_deg.to_radians()
    }
}

mod example_entities {
    use crate::*;
    use notan::math::Vec3;
    struct DOOMCacodemon {
        pos: Vec3,
        angle_deg: f32,
    }

    impl NonPlayerEntity for DOOMCacodemon {
        fn get_pos(&self) -> notan::math::Vec3 {
            self.pos
        }

        fn get_angle_deg(&self) -> f32 {
            self.angle_deg
        }

        fn get_angle_rad(&self) -> f32 {
            self.angle_deg.to_radians()
        }

        fn translate(&mut self, translation: Vec3) {
            self.pos = self.pos + translation
        }

        fn attack(&mut self) {
            todo!()
        }

        fn do_movement(&mut self) {
            todo!()
        }

        fn is_attacking(&self) -> bool {
            todo!()
        }
    }

    impl Animated for DOOMCacodemon {
        fn idle_animation(&self) -> Animation {
            vec![Box::new(*include_bytes!("./assets/doom_cacodemon.png"))]
        }

        fn attack_animation(&self) -> Animation {
            self.idle_animation()
        }

        fn reload_animation(&self) -> Animation {
            self.idle_animation()
        }

        fn walk_animation(&self) -> Animation {
            self.idle_animation()
        }
    }
}