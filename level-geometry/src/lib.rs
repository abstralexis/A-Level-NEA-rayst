use anyhow::anyhow;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// The trait that defines the properties of a line.
pub trait Line {
    /// The points that define the x, y, and z coordinates of
    /// the two bottom corners of the wall.
    fn points(&self) -> (Vec3, Vec3);
    /// Height of the wall above the bottom coordinates
    fn height(&self) -> f32;
    fn new(points: (Vec3, Vec3), height: f32) -> Self;
    /// Splits the current line by another line as a hyperplane
    /// should they intersect. Checks if parallel, and if the calculated
    /// intersection point falls beyond the bounds of the wall
    /// defined in self.
    fn split_by<T: Line>(&self, line: T) -> Option<(Seg, Seg)>;
    /// Checks if this line is parallel to another line based
    /// off of an equation generated for each line according
    /// to their x and z coordinates.
    fn intersects<T: Line>(&self, line: T) -> bool;
    /// Get the coordinates of all four corners of the wall
    /// defined by the current line.
    fn get_corners(&self) -> (Vec3, Vec3, Vec3, Vec3);
    /// Returns the gradient of the current line for the x, z plane
    fn xz_gradient(&self) -> f32;
}

/// A seg is a portion of a linedef
pub struct Seg {
    points: (Vec3, Vec3),
    height: f32,
}
impl Line for Seg {
    fn new(points: (Vec3, Vec3), height: f32) -> Self {
        Seg { points, height }
    }

    fn height(&self) -> f32 {
        self.height
    }

    fn points(&self) -> (Vec3, Vec3) {
        self.points
    }

    fn get_corners(&self) -> (Vec3, Vec3, Vec3, Vec3) {
        (
            self.points.0,
            self.points.1,
            self.points.0 + Vec3::from((0f32, 0f32, self.height)),
            self.points.1 + Vec3::from((0f32, 0f32, self.height)),
        )
    }

    fn split_by<T: Line>(&self, line: T) -> Option<(Seg, Seg)> {
        todo!()
    }

    fn intersects<T: Line>(&self, line: T) -> bool {
        todo!()
    }

    fn xz_gradient(&self) -> f32 {
        (self.points.1.z - self.points.0.z) / (self.points.1.x - self.points.0.x)
    }
}
