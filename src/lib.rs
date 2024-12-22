use glam::Vec3;

pub mod core;

struct Transform {
    forward: Vec3,
    up: Vec3,
    right: Vec3,
    position: Vec3,
    rotation: Vec3,
}

struct Model {
    vao: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>,
}