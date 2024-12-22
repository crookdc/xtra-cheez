use std::collections::HashSet;
use gl::types::{GLint, GLvoid};
use glam::{Mat4, Vec3};
use image::ImageReader;
use psx::core::render::shader::Shader;
use sdl2::video::GLProfile;
use std::f32::consts::PI;
use std::time::SystemTime;
use sdl2::keyboard::Scancode;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("psx", 800, 800)
        .opengl()
        .build()
        .unwrap();
    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    sdl_context.mouse().set_relative_mouse_mode(true);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    let shader_program = unsafe {
        Shader::from_source_files("shaders/vertex.glsl", "shaders/fragment.glsl").unwrap()
    };

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        debug_assert_ne!(vao, 0);
    }

    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        debug_assert_ne!(vbo, 0);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let vertices: [f32; 180] = [
            -0.5, -0.5, -0.5,  0.0, 0.0,
            0.5, -0.5, -0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5,  0.5,  0.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  1.0, 1.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
        ];
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * size_of::<f32>()) as GLint,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * size_of::<f32>()) as GLint,
            (3 * size_of::<f32>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }

    let texture = unsafe {
      let img = ImageReader::open("assets/textures/bricks.jpg").unwrap().decode().unwrap();
      let img_rgb = img.to_rgb8().as_raw().to_owned();
      let mut texture_id = 0;
      gl::GenTextures(1, &mut texture_id);
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_2D, texture_id);
      debug_assert_ne!(texture_id, 0);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
      gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, img.width() as i32, img.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, img_rgb.as_ptr() as *const _);
      gl::GenerateMipmap(gl::TEXTURE_2D);
      texture_id
    };

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let inception = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let mut tick = inception;
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut camera_position = Vec3::new(0.0, 0.0, -3.0);
    let camera_speed = 0.05;
    let mut camera_front = Vec3::new(0.0, 0.0, -1.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let mut camera_rotation = Vec3::new(0.0, 0.0, -90.0);
    let mut keymap: HashSet<Scancode> = HashSet::new();
    'main: loop {
        let next = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f64();
        tick = next - inception;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { scancode, .. } => {
                    keymap.insert(scancode.unwrap());
                },
                sdl2::event::Event::KeyUp { scancode, .. } => {
                    keymap.remove(&scancode.unwrap());
                },
                sdl2::event::Event::MouseMotion {xrel, yrel, ..} => {
                    camera_rotation.x += xrel as f32 * 0.5;
                    camera_rotation.y -= yrel as f32 * 0.5;
                }
                _ => {}
            }
        }
        if keymap.contains(&Scancode::Escape) {
            break 'main;
        }

        let mut camera_movement_direction = Vec3::new(0.0, 0.0, 0.0);
        if keymap.contains(&Scancode::W) {
            camera_movement_direction += camera_front;
        }
        if keymap.contains(&Scancode::S) {
            camera_movement_direction -= camera_front;
        }
        if keymap.contains(&Scancode::A) {
            camera_movement_direction += camera_up.cross(camera_front).normalize();
        }
        if keymap.contains(&Scancode::D) {
            camera_movement_direction -= camera_up.cross(camera_front).normalize();
        }
        camera_position += camera_movement_direction * camera_speed;
        unsafe {
            gl::ClearColor(0.1, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader_program.use_program();

            let projection = Mat4::perspective_rh(radians(45.0), 800.0/600.0, 0.1, 100.0);
            shader_program.set_mat4("projection", &projection);

            let model = Mat4::IDENTITY.clone();
            shader_program.set_mat4("model", &model);

            let mut camera_direction = Vec3::default();
            camera_direction.x = f32::cos(radians(camera_rotation.x)) * f32::cos(radians(camera_rotation.y));
            camera_direction.y = f32::sin(radians(camera_rotation.y));
            camera_direction.z = f32::sin(radians(camera_rotation.x)) * f32::cos(radians(camera_rotation.y));
            camera_front = camera_direction.normalize();

            let view = Mat4::look_at_rh(camera_position, camera_position + camera_front, camera_up);
            shader_program.set_mat4("view", &view);

            shader_program.set_int("texture2", 1);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
        }
        window.gl_swap_window();
    }
}

fn radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}