use notan::prelude::*;

mod weapons;
mod entities;
mod animation;

use crate::weapons::*;
use crate::entities::*;
use crate::animation::*;

struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vbo: Buffer, // Vertex Buffer
    ibo: Buffer, // Index Buffer
    ubo: Buffer, // Uniform Buffer
    mvp: notan::math::Mat4, // Model View Projection Matrix
    fov: f32,
    player: Player,
    animation_max_fps: usize,
}

#[notan_main]
fn main() {
    println!("Hello, world!");
}
