use crate::core::radians;
use crate::core::render::model::{Material, Mesh};
use glam::Vec3;
use sdl2::keyboard::Scancode;

#[derive(Copy, Clone, Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            f32::cos(radians(self.rotation.x)) * f32::sin(radians(self.rotation.y)),
            -f32::sin(radians(self.rotation.x)),
            f32::cos(radians(self.rotation.x)) * f32::cos(radians(self.rotation.y)),
        )
        .normalize()
    }

    pub fn right(&self) -> Vec3 {
        Vec3::new(-f32::cos(self.rotation.y), 0.0, f32::sin(self.rotation.y)).normalize()
    }

    pub fn left(&self) -> Vec3 {
        self.right() * -1.0
    }

    pub fn up(&self) -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    pub fn distance(&self, other: Self) -> f32 {
        self.position.distance(other.position)
    }
}

#[derive(Copy, Clone)]
pub struct KeyboardControls {
    pub forward: Scancode,
    pub backward: Scancode,
    pub left: Scancode,
    pub right: Scancode,
}

impl Default for KeyboardControls {
    fn default() -> Self {
        Self {
            forward: Scancode::W,
            backward: Scancode::S,
            left: Scancode::A,
            right: Scancode::D,
        }
    }
}

#[derive(Clone)]
pub struct Model {
    pub materials: Vec<Material>,
    pub vao: u32,
}

impl Model {
    pub fn new(mesh: Mesh) -> Self {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        }
        let vertices = mesh.serialize();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (size_of::<f32>() * vertices.len()) as isize,
                vertices.as_slice().as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (5 * size_of::<f32>()) as gl::types::GLint,
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * size_of::<f32>()) as gl::types::GLint,
                (3 * size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        Self {
            vao,
            materials: mesh.materials,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Lens {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Lens {
    fn default() -> Self {
        Self {
            fov: radians(45.0),
            aspect_ratio: 800.0 / 600.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct CameraTarget(pub f32);