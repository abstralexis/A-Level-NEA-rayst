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
}

mod tests {
    use crate::*;

    /// An initialisation function for the tests so I do not
    /// have to rewrite the same definitions over and over.
    /// 0 is the default, 1 is parallel, 2 will intersect.
    /// Notably though, 3 would intersect based off of gradient but
    /// the lengths mean a bug occurs. Should we combat this with checks
    /// or assume that we will only be using it with lines we know are not
    /// wholly in front nor wholly behind? That would make no sense of course,
    /// because we already check for whether it is parallel.   
    #[allow(dead_code)]
    fn init() -> (Seg, Seg, Seg, Seg) {
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
}
