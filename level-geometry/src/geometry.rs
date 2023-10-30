//! The structs and functions for describing level geometry

use std::{f32::consts::PI, mem::swap};
use crate::*;

/// Enum with types to represent the side
/// determination that we need for
/// binary space partitioning.
#[derive(PartialEq, Debug)]
pub enum Side {
    Front,
    Back,
    Neither,
}

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
    fn split_by<T: Line + Clone>(&self, line: T) -> Option<(Seg, Seg)>;
    /// Checks if this line is parallel to another line based
    /// off of an equation generated for each line according
    /// to their x and z coordinates.
    fn intersects<T: Line>(&self, line: T) -> bool;
    /// Get the coordinates of all four corners of the wall
    /// defined by the current line.
    fn get_corners(&self) -> (Vec3, Vec3, Vec3, Vec3);
    /// Returns the gradient of the current line for the x, z plane
    fn xz_gradient(&self) -> f32;
    /// Return the normal unit vector to self
    fn normal(&self) -> Vec3;
    /// Swap the point order of the line
    fn flip(&mut self);
    /// Determine which side of self that other is on relative
    /// to self's surface normal (front side)
    fn determine_side<T: Line>(&self, other: T) -> Side;
}

/// A seg is a portion of a linedef
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
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
        self.xz_gradient() != line.xz_gradient()
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

                // Make sure the intersect is real
                match self.points().0.x < intersection_point.x
                    && intersection_point.x < self.points().1.x
                {
                    false => None,
                    true => match self.points().0.z < intersection_point.z
                        && intersection_point.z < self.points().1.z
                    {
                        false => None,
                        true => Some((
                            Seg::new((self.points().0, intersection_point), self.height),
                            Seg::new((intersection_point, self.points().1), self.height),
                        )),
                    },
                }
            }
        }
    }

    fn normal(&self) -> Vec3 {
        // Get the vector defining self based off of the first point
        let line_vector = self.points().1 - self.points().0;
        // Rotate that vector using the y axis by 90 degrees
        let rotation = glam::Mat3::from_rotation_y(PI / 2f32);
        let perpendicular: Vec3 = rotation * line_vector;
        // Return the unit vector of this vector
        perpendicular.normalize_or_zero()
    }

    fn flip(&mut self) {
        swap(&mut self.points.0, &mut self.points.1);
    }

    fn determine_side<T: Line>(&self, other: T) -> Side {
        // Evaluate a vector for self relative to the first point
        let self_vec = (self.points.1 - self.points.0).normalize_or_zero();
        // Evaluate a vector for both points of other relative
        // to our first point
        let other_0 = (other.points().0 - self.points.0).normalize_or_zero();
        let other_1 = (other.points().1 - self.points.0).normalize_or_zero();
        // Evaluate the cross products of self with both other points
        let cross_0 = self_vec.cross(other_0);
        let cross_1 = self_vec.cross(other_1);

        // Both zero
        if (cross_0.y, cross_1.y) == (0f32, 0f32) {
            Side::Front
        // One greater than zero, one equal to zero
        } else if ((cross_0.y < 0f32) && (cross_1.y == 0f32))
            || ((cross_0.y == 0f32) && (cross_1.y < 0f32))
        {
            Side::Back
        // One less than zero, one equal to zero
        } else if ((cross_0.y > 0f32) && (cross_1.y == 0f32))
            || ((cross_0.y == 0f32) && (cross_1.y > 0f32))
        {
            Side::Front
        // Both greater than zero
        } else if (cross_0.y < 0f32) && (cross_1.y < 0f32) {
            Side::Back
        // Both less than zero
        } else if (cross_0.y > 0f32) && (cross_1.y > 0f32) {
            Side::Front
        // If none of the previous cases, then must be one positive
        // and one negative => Neither.
        } else {
            Side::Neither
        }
    }
}

pub mod tests {
    use crate::geometry::*;

    /// An initialisation function for the tests so I do not
    /// have to rewrite the same definitions over and over.
    /// 0 is the default, 1 is parallel, 2 will intersect.
    /// Notably though, 3 would intersect based off of gradient but
    /// the lengths could mean a bug occurs. We have now combatted this
    /// with a unit test for it and a fix to make the test pass.
    #[allow(dead_code)]
    pub fn init() -> (Seg, Seg, Seg, Seg) {
        let height = 10f32;
        (
            Seg::new(
                (
                    Vec3::from((0f32, 0f32, 0f32)),
                    Vec3::from((1f32, 0f32, 1f32)),
                ),
                height,
            ),
            Seg::new(
                (
                    Vec3::from((1f32, 0f32, 0f32)),
                    Vec3::from((2f32, 0f32, 1f32)),
                ),
                height,
            ),
            Seg::new(
                (
                    Vec3::from((0f32, 0f32, 1f32)),
                    Vec3::from((2f32, 0f32, 0f32)),
                ),
                height,
            ),
            Seg::new(
                (
                    Vec3::from((0f32, 0f32, -1f32)),
                    Vec3::from((1f32, 0f32, -2f32)),
                ),
                height,
            ),
        )
    }

    #[test]
    fn parallel() {
        let (seg0, seg1, _, _) = init();
        assert!(seg0.intersects(seg1) == false)
    }

    #[test]
    fn not_parallel() {
        let (seg0, _, seg2, _) = init();
        assert!(seg0.intersects(seg2) == true)
    }

    #[test]
    fn split_parallel() {
        let (seg0, seg1, _, _) = init();
        assert!(seg0.split_by(seg1).is_none() == true)
    }

    #[test]
    fn split() {
        let height = 10f32;
        let (seg0, _, seg2, _) = init();
        let should_intersects_at = Vec3::from((2f32 / 3f32, 0f32, 2f32 / 3f32));
        let (shouldnewseg0, shouldnewseg1) = (
            Seg::new(
                (Vec3::from((0f32, 0f32, 0f32)), should_intersects_at),
                height,
            ),
            Seg::new(
                (should_intersects_at, Vec3::from((1f32, 0f32, 1f32))),
                height,
            ),
        );
        let (newseg0, newseg1) = seg0.split_by(seg2).unwrap();
        assert!((newseg0, newseg1) == (shouldnewseg0, shouldnewseg1))
    }

    #[test]
    fn no_actual_intersect() {
        let (seg0, _, _, seg3) = init();
        let split = seg0.split_by(seg3);
        dbg!(&split);
        assert!(split.is_none() == true)
    }

    #[test]
    fn flip() {
        let (mut seg0, _, _, _) = init();
        seg0.flip();
        assert!(
            seg0 == Seg::new(
                (
                    Vec3::from((1f32, 0f32, 1f32)),
                    Vec3::from((0f32, 0f32, 0f32)),
                ),
                10f32,
            )
        )
    }

    #[test]
    fn normal_swap() {
        let (seg0, _, _, _) = init();

        let mut flip_seg0 = seg0.clone();
        flip_seg0.flip();

        assert!(seg0.normal() != flip_seg0.normal())
    }

    #[test]
    fn front() {
        let (seg0, seg1, _, _) = init();
        assert!(seg0.determine_side(seg1) == Side::Front)
    }

    #[test]
    fn back() {
        let (mut seg0, seg1, _, _) = init();
        seg0.flip();
        assert!(seg0.determine_side(seg1) == Side::Back)
    }

    #[test]
    fn neither() {
        let (seg0, _, seg2, _) = init();
        assert!(seg0.determine_side(seg2) == Side::Neither)
    }
}
