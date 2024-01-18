//! Animation code and traits

/// Animation type alias
pub type Animation = Vec<Box<[u8]>>;

pub trait Animated {
    /// Looping animation for when idle
    fn idle_animation(&self) -> Animation;
    /// Looping animation for when walking
    fn walk_animation(&self) -> Animation;
    /// Animation for when reloading. Reload lasts as long as animation.
    fn reload_animation(&self) -> Animation;
    /// Animation for when firing. Loops if `FireMode` is set to `Auto`, else 
    /// the firing speed is dictated by the length of the animation.
    fn attack_animation(&self) -> Animation;
}