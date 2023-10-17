use anyhow::anyhow;
use glam::Vec3;
use serde::{Serialize, Deserialize};

pub trait Line {
    fn points(&self) -> (Vec3, Vec3);
    fn height(&self) -> f32;
    fn new(points: (Vec3, Vec3), height: f32);
    fn split_by<T: Line>(&self, line: T) -> Option<Seg, Seg>;
    fn intersects<T: Line>(&self, line: T) -> bool; 
}