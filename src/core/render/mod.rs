use crate::core::ecs::component::{CameraTarget, Lens, Model, Transform};
use crate::core::ecs::{Query, ECS};
use crate::core::render::shader::Shader;
use crate::core::{radians, Mouse};
use glam::{Mat4, Vec3};

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
    let shader_id = ecs.get_resource::<Shader>().unwrap().get_id();
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
        let model_matrix = model_matrix(&ecs.clone_component::<Transform>(id).unwrap());
        unsafe {
            gl::UseProgram(shader_id);
            shader::set_mat4(shader_id, "projection", &projection_matrix);
            shader::set_mat4(shader_id, "view", &view_matrix);
            shader::set_mat4(shader_id, "model", &model_matrix);
            gl::BindVertexArray(model.vao);
            for material in model.materials {
                let texture_id = material.texture_id.or(Some(0)).unwrap();
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
                gl::DrawArrays(gl::TRIANGLES, material.first_index, material.count);
            }
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindVertexArray(0);
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
    Mat4::look_at_rh(
        camera_position,
        target.position + target.pivot,
        transform.up(),
    )
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
        * Mat4::from_scale(transform.scale)
        * Mat4::from_translation(transform.pivot)
        * Mat4::from_rotation_x(radians(transform.rotation.x))
        * Mat4::from_rotation_y(radians(transform.rotation.y))
        * Mat4::from_rotation_z(radians(transform.rotation.z))
        * Mat4::from_translation(-transform.pivot)
}

pub fn build_camera(ecs: &mut ECS) {
    let id = ecs.create_entity();
    ecs.attach_component(
        id,
        Transform {
            position: Vec3::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            pivot: Vec3::default(),
            rotation: Vec3::new(-90.0, 0.0, 0.0),
        },
    )
    .unwrap();
    ecs.attach_component(id, Lens::default()).unwrap();
}

pub fn camera_movement_system(ecs: &mut ECS, delta_time: f32) {
    let camera = ecs.query(&Query::new().with::<Transform>().with::<Lens>().build())[0];
    let x_rel = ecs.get_resource::<Mouse>().unwrap().0;
    ecs.update_component::<Transform>(camera, &mut |mut transform| {
        transform.rotation.x += 4.0 * x_rel as f32 * delta_time;
        transform
    })
    .unwrap();
}
