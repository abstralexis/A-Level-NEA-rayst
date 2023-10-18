#[allow(unused_imports)]
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
    fn split_by<T: Line+ Clone>(&self, line: T) -> Option<(Seg, Seg)>;
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
#[derive(Clone, Copy, Serialize, Deserialize)]
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
    
    fn intersects<T: Line>(&self, line: T) -> bool {
        self.xz_gradient() == line.xz_gradient()
    }

    fn xz_gradient(&self) -> f32 {
        (self.points.1.z - self.points.0.z) / (self.points.1.x - self.points.0.x)
    }

    fn split_by<T: Line + Clone>(&self, line: T) -> Option<(Seg, Seg)> {
        match self.intersects(line.clone()) {
            false => None,
            true => {
                let x = (-self.xz_gradient() * self.points().0.x
                    + line.xz_gradient() * self.points().0.x
                    + self.points().0.z
                    - line.points().0.z)
                    / (line.xz_gradient() - self.xz_gradient());
                let z = self.xz_gradient() * (x - self.points().0.x) + self.points().0.z;
                let y = self.points().0.y;
                let intersection_point = Vec3::from((x, y, z));

                Some (
                    (
                        Seg::new((self.points().0, intersection_point), self.height),
                        Seg::new((intersection_point, self.points().1), self.height)
                    )
                ) 
            }
        }
    }
}
