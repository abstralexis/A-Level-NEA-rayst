//! The weapons module for Rayst
use notan::math::{Vec3, Vec2};
use crate::animation::*;

/// Enum used for dictating whether a weapon is
/// single fire or rapid fire
pub enum FireMode {
    Semi,
    Auto,
}

/// Attack type structure
pub struct Attack {
    damage: f32,
    /// Formatted function like dropoff(distance) and returns a multiplier
    range_dropoff: Box<dyn Fn(f32) -> f32>, // Damage dropoff function 
}

/// Weapon trait for Rayst Engine
pub trait Weapon {
    /// Gets the relevant attack structure
    fn get_attack(&self) -> Attack;
}

mod example_weapons {
    use std::f32::consts::PI;
    use crate::{animation::*, *};
    pub struct Pistol;

    impl Pistol {
        fn dropoff(distance: f32) -> f32 {
            let stretch = 0.05;
            if distance >= ((1.0/stretch) * PI) {
                0.1
            } else {
                0.5 * ((distance * stretch).cos()) + 0.6
            }
        }
    }

    impl Weapon for Pistol {
        fn get_attack(&self) -> Attack {
            Attack { damage: 12.0, range_dropoff: Box::new(Pistol::dropoff) }
        }
    }

    impl Animated for Pistol {
        fn idle_animation(&self) -> Animation {
            vec![Box::new(*include_bytes!("./assets/doom_cacodemon.png"))]
        }

        fn walk_animation(&self) -> Animation {
            self.idle_animation()
        }

        fn reload_animation(&self) -> Animation {
            self.idle_animation()
        }

        fn attack_animation(&self) -> Animation {
            self.idle_animation()
        }
    }
}