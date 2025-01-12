use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::video::GLProfile;
use std::collections::HashSet;
use std::time::SystemTime;
use xtra_cheez::core::ecs::component::{
    CameraTarget, KeyboardControls, Lens, Model, Transform,
};
use xtra_cheez::core::ecs::ECSBuilder;
use xtra_cheez::core::physics::{DynamicPhysicsBody, PhysicsBody};
use xtra_cheez::core::render::model::MeshLoader;
use xtra_cheez::core::render::shader::Shader;
use xtra_cheez::core::render::Color;
use xtra_cheez::core::{physics, render, Keymap, Mouse};
use xtra_cheez::gameplay;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("XTRA CHEEZ", 800, 800)
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
        .with_component::<DynamicPhysicsBody>()
        .with_component::<PhysicsBody>()
        .with_resource(Keymap(HashSet::new()))
        .with_resource(Mouse(0, 0))
        .with_resource(
            Shader::from_source_files("assets/shaders/vertex.glsl", "assets/shaders/fragment.glsl")
                .unwrap(),
        )
        .with_resource(MeshLoader::new())
        .build();

    render::build_camera(&mut ecs);
    gameplay::build_player(&mut ecs);

    let maze = gameplay::generate_cityscape(10, 10);
    gameplay::build_entities(&mut ecs, &maze);

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

        render::move_camera(&mut ecs, delta_time);
        gameplay::move_player(&mut ecs, delta_time);

        physics::collision_system(&mut ecs, delta_time);
        physics::velocity_system(&mut ecs, delta_time);

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
