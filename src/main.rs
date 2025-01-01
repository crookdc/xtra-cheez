use glam::Vec3;
use pizza_delivery::core::ecs::component::{
    CameraTarget, KeyboardControls, Lens, Material, Model, PhysicsBody, Transform,
};
use pizza_delivery::core::ecs::{ECSBuilder, ECS};
use pizza_delivery::core::render::model::parse_obj_file;
use pizza_delivery::core::render::shader::Shader;
use pizza_delivery::core::render::Color;
use pizza_delivery::core::{physics, render, Keymap, Mouse};
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::video::GLProfile;
use std::collections::HashSet;
use std::time::SystemTime;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Pizza Delivery", 800, 800)
        .opengl()
        .build()
        .unwrap();
    let _ctx = window.gl_create_context().unwrap();
    sdl_context.mouse().set_relative_mouse_mode(true);
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }

    let mut ecs = ECSBuilder::new()
        .with_component::<Transform>()
        .with_component::<Lens>()
        .with_component::<Model>()
        .with_component::<CameraTarget>()
        .with_component::<KeyboardControls>()
        .with_component::<PhysicsBody>()
        .with_resource(Keymap(HashSet::new()))
        .with_resource(Mouse(0, 0))
        .with_resource(
            Shader::from_source_files("assets/shaders/vertex.glsl", "assets/shaders/fragment.glsl")
                .unwrap(),
        )
        .build();
    camera::build(&mut ecs);
    player::build(&mut ecs);
    build_tiles(&mut ecs);
    let mut events = sdl_context.event_pump().unwrap();
    let mut tick = SystemTime::now();
    'game: loop {
        let delta_time = loop {
            let now = SystemTime::now();
            let delta_time = now.duration_since(tick).unwrap().as_secs_f32();
            // Cap the frame rate to 60 FPS
            if delta_time >= 0.016 {
                break delta_time;
            }
        };
        tick = SystemTime::now();
        ecs.get_resource_mut::<Mouse>().unwrap().reset();
        for event in events.poll_iter() {
            if quit(&event) {
                break 'game;
            }
            ecs.get_resource_mut::<Keymap>().unwrap().consume(&event);
            ecs.get_resource_mut::<Mouse>().unwrap().consume(&event);
        }

        camera::movement_system(&mut ecs, delta_time);
        player::movement_system(&mut ecs, delta_time);
        physics::system(&mut ecs, delta_time);

        render::clear(&Color(0.0, 0.05, 0.05, 1.0));
        render::draw(&mut ecs);
        window.gl_swap_window();
    }
}

fn quit(event: &Event) -> bool {
    match event {
        Event::Quit { .. } => true,
        Event::KeyUp {
            scancode: Some(Scancode::Escape),
            ..
        } => true,
        _ => false,
    }
}

mod camera {
    use glam::Vec3;
    use pizza_delivery::core::ecs::component::{Lens, Transform};
    use pizza_delivery::core::ecs::{Query, ECS};
    use pizza_delivery::core::Mouse;

    pub fn build(ecs: &mut ECS) {
        let id = ecs.create_entity();
        ecs.attach_component(
            id,
            Transform {
                position: Vec3::default(),
                pivot: Vec3::default(),
                rotation: Vec3::new(-90.0, 0.0, 0.0),
            },
        )
        .unwrap();
        ecs.attach_component(id, Lens::default()).unwrap();
    }

    pub fn movement_system(ecs: &mut ECS, delta_time: f32) {
        let camera = ecs.query(&Query::new().with::<Transform>().with::<Lens>().build())[0];
        let x_rel = ecs.get_resource::<Mouse>().unwrap().0;
        ecs.update_component::<Transform>(camera, &mut |mut transform| {
            transform.rotation.x += 4.0 * x_rel as f32 * delta_time;
            transform
        })
        .unwrap();
    }
}

mod player {
    use glam::Vec3;
    use pizza_delivery::core::ecs::component::{
        CameraTarget, KeyboardControls, Material, Model, PhysicsBody, Transform,
    };
    use pizza_delivery::core::ecs::{Query, ECS};
    use pizza_delivery::core::render::model;
    use pizza_delivery::core::render::model::parse_obj_file;
    use pizza_delivery::core::render::shader::Shader;
    use pizza_delivery::core::Keymap;

    pub fn build(ecs: &mut ECS) {
        let id = ecs.create_entity();
        ecs.attach_component(
            id,
            Transform {
                position: Vec3::default(),
                pivot: Vec3::new(0.0, 0.0, -2.5),
                rotation: Vec3::new(0.0, 0.0, 0.0),
            },
        )
        .unwrap();
        ecs.attach_component(
            id,
            Model::new(
                Material {
                    shader_id: ecs.get_resource::<Shader>().unwrap().get_id(),
                    texture_id: Some(model::load_texture("assets/textures/player.png")),
                },
                &parse_obj_file("assets/models/player.obj").unwrap(),
            ),
        )
        .unwrap();
        ecs.attach_component(id, CameraTarget()).unwrap();
        ecs.attach_component(id, KeyboardControls::default())
            .unwrap();
        ecs.attach_component(
            id,
            PhysicsBody {
                force: Vec3::default(),
                velocity: Vec3::default(),
                mass: 1.0,
            },
        )
        .unwrap()
    }

    pub fn movement_system(ecs: &mut ECS, delta_time: f32) {
        let id = ecs.query(
            &Query::new()
                .with::<KeyboardControls>()
                .with::<Transform>()
                .with::<PhysicsBody>(),
        )[0];
        let controls = ecs.clone_component::<KeyboardControls>(id).unwrap();
        let transform = ecs.clone_component::<Transform>(id).unwrap();

        let drive_dir = ecs
            .get_resource::<Keymap>()
            .unwrap()
            .axis(controls.forward, controls.backward);
        ecs.update_component::<PhysicsBody>(id, &mut |mut body| {
            if drive_dir == 0.0 {
                body.force -= body.velocity;
            } else {
                body.force += transform.forward() * 30.0 * drive_dir * delta_time;
            }
            body
        })
        .unwrap();
        let velocity = ecs
            .get_component::<PhysicsBody>(id)
            .unwrap()
            .borrow()
            .downcast_ref::<PhysicsBody>()
            .unwrap()
            .velocity
            .clone();
        let steer_dir = ecs
            .get_resource::<Keymap>()
            .unwrap()
            .axis(controls.left, controls.right);
        ecs.update_component::<Transform>(id, &mut |mut transform| {
            if velocity.length().abs() < 0.025 {
                return transform;
            }
            transform.rotation.y += 10.0 / velocity.length() * delta_time * steer_dir;
            transform
        })
        .unwrap()
    }
}

fn build_tiles(ecs: &mut ECS) {
    let model = Model::new(
        Material {
            shader_id: ecs.get_resource::<Shader>().unwrap().get_id(),
            texture_id: None,
        },
        &parse_obj_file("assets/models/tile.obj").unwrap(),
    );
    for i in 0..40 {
        for j in 0..40 {
            let id = ecs.create_entity();
            ecs.attach_component(
                id,
                Transform {
                    position: Vec3::new((j * 3) as f32, 0.0, (i * 3) as f32),
                    pivot: Vec3::default(),
                    rotation: Vec3::default(),
                },
            )
            .unwrap();
            ecs.attach_component(id, model).unwrap()
        }
    }
}
