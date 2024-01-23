use std::default::Default;
use std::f32::consts::PI;

use notan::prelude::*;
use notan::math::{Mat4, Vec3};

mod weapons;
mod entities;
mod animation;

use crate::weapons::*;
use crate::entities::*;
use crate::animation::*;

// Arbitrary constants for the view matrix
const FOV: f32 = (2.0*PI)/3.0;
const ASPECT_RATIO: f32 = 16.0/9.0;
const Z_NEAR: f32 = 0.0;
const Z_FAR: f32 = 255.0; 

// notan example shaders
// takes in the position and colour like a sliding window
// over the array of values passed in. It uses a uniform buffer
// Locals declared later to pass in a modelview projection.
// It does not alter the colour, but it does perform matrix transformations
// on the gpu which is a lot faster than on the cpu.
//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec4 a_position;
    layout(location = 1) in vec4 a_color;

    layout(location = 0) out vec4 v_color;

    layout(set = 0, binding = 0) uniform Locals {
        mat4 u_matrix;
    };

    void main() {
        v_color = a_color;
        gl_Position = u_matrix * a_position;
    }
    "#
};

// Another notan example. Fairly self explanatory boilerplate, you
// don't usually get much different to this.
//language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec4 v_color;
    layout(location = 0) out vec4 color;

    void main() {
        color = v_color;
    }
    "#
};

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

impl State {
    pub fn init(gfx: &mut Graphics) -> Self {
        let mut player = Player {
            pos: Vec3::default(),
            weapon: Box::new(example_weapons::Pistol{}),
            angle_deg: Default::default(),
        };

        let clear_options = ClearOptions {
            color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
            depth: Some(1.0),
            stencil: None,
        };

        // This will take care of the remaining fragments that are overlapping. 
        let stencil = DepthStencil {
            write: true,
            compare: CompareMode::Less
        };

        let vertex_info = VertexInfo::new()
            .attr(0, VertexFormat::Float32x3) // Position
            .attr(1, VertexFormat::Float32x4); // Colour

        let pipeline = gfx.create_pipeline()
            .from(&VERT, &FRAG)
            .with_vertex_info(&vertex_info)
            .with_depth_stencil(stencil)
            .build()
            .unwrap();

        // I need to make something to unwrap verts and idcs from map data
        let vertices = todo!();
        let indices = todo!();

        let projection = Mat4::perspective_rh_gl(FOV, ASPECT_RATIO, Z_NEAR, Z_FAR);
    }
}

#[notan_main]
fn main() {
    println!("Hello, world!");
}
