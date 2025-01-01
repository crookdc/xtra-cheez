use crate::core::ecs::component::{CameraTarget, Lens, Model, Transform};
use crate::core::ecs::{Query, ECS};
use crate::core::radians;
use glam::{Mat4, Vec3};
use sdl2::libc::pipe;
use std::f32::consts::PI;

pub mod model;
pub mod shader;

pub struct Color(pub f32, pub f32, pub f32, pub f32);

pub fn clear(color: &Color) {
    unsafe {
        gl::ClearColor(color.0, color.1, color.2, color.3);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub fn draw(ecs: &mut ECS) {
    let camera = *ecs
        .query(&Query::new().with::<Lens>().with::<Transform>().build())
        .first()
        .unwrap();
    let projection_matrix = projection_matrix(
        ecs.get_component::<Lens>(camera)
            .unwrap()
            .borrow()
            .downcast_ref::<Lens>()
            .unwrap(),
    );
    let target = ecs
        .clone_component::<Transform>(
            ecs.query(
                &Query::new()
                    .with::<CameraTarget>()
                    .with::<Transform>()
                    .build(),
            )[0],
        )
        .unwrap();
    let view_matrix = targeted_view_matrix(
        ecs.get_component::<Transform>(camera)
            .unwrap()
            .borrow()
            .downcast_ref::<Transform>()
            .unwrap(),
        &target,
    );
    let models = ecs.query(&Query::new().with::<Transform>().with::<Model>().build());
    for id in models {
        let model = ecs.clone_component::<Model>(id).unwrap();
        let model_matrix = model_matrix(
            &ecs.clone_component::<Transform>(id).unwrap(),
        );
        unsafe {
            gl::UseProgram(model.material.shader_id);
            shader::set_mat4(model.material.shader_id, "projection", &projection_matrix);
            shader::set_mat4(model.material.shader_id, "view", &view_matrix);
            shader::set_mat4(model.material.shader_id, "model", &model_matrix);
            if let Some(texture) = model.material.texture_id {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                shader::set_int(model.material.shader_id, "u_texture", texture as i32);
            }
            gl::BindVertexArray(model.vertex_array_object);
            gl::DrawArrays(gl::TRIANGLES, 0, model.vertex_count as i32);
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

pub fn projection_matrix(lens: &Lens) -> Mat4 {
    Mat4::perspective_rh(lens.fov, lens.aspect_ratio, lens.near, lens.far)
}

pub fn targeted_view_matrix(transform: &Transform, target: &Transform) -> Mat4 {
    let mut camera_position = target.position.clone() + target.pivot;
    camera_position.y = 4.0;
    camera_position.x += f32::cos(radians(transform.rotation.x)) * 12.0;
    camera_position.z += f32::sin(radians(transform.rotation.x)) * 12.0;
    Mat4::look_at_rh(camera_position, target.position + target.pivot, transform.up())
}

pub fn free_view_matrix(transform: &Transform) -> Mat4 {
    Mat4::look_at_rh(
        transform.position,
        transform.position + transform.forward(),
        transform.up(),
    )
}

pub fn model_matrix(transform: &Transform) -> Mat4 {
    Mat4::from_translation(transform.position)
        * Mat4::from_translation(transform.pivot)
        * Mat4::from_rotation_x(radians(transform.rotation.x))
        * Mat4::from_rotation_y(radians(transform.rotation.y))
        * Mat4::from_rotation_z(radians(transform.rotation.z))
        * Mat4::from_translation(-transform.pivot)
}
