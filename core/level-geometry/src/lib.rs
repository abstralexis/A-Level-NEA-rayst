//! The level geometry library contains code for
//! defining the geometry as well as compiling it
//! into serialised data of binary space partitioned
//! geometry.

#[allow(unused_imports)]
use anyhow::anyhow;
use glam::Vec3;
use serde::{Deserialize, Serialize};

pub mod geometry;
pub mod partitioning;