use glam::{Vec2, Vec3};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use image::ImageReader;

#[derive(Default)]
pub struct Mesh {
    triangles: Vec<[Vertex; 3]>,
}

impl Mesh {
    pub fn serialize(&self) -> Vec<f32> {
        let mut buffer = vec![];
        for triangle in &self.triangles {
            for vertex in triangle {
                buffer.push(vertex.position.x);
                buffer.push(vertex.position.y);
                buffer.push(vertex.position.z);
                buffer.push(vertex.texture_coordinate.x);
                buffer.push(vertex.texture_coordinate.y);
            }
        }
        buffer
    }

    pub fn vertex_length(&self) -> u32 {
        (self.triangles.len() * 3) as u32
    }
}

#[derive(Default)]
pub struct Vertex {
    position: Vec3,
    texture_coordinate: Vec2,
}

pub fn parse_obj_file(file_path: &str) -> Result<Mesh, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut vertices: Vec<Vec3> = vec![];
    let mut texture_coordinates: Vec<Vec2> = vec![];
    let mut faces: Vec<[Vertex; 3]> = vec![];
    for line in reader.lines() {
        let line = line?;
        let mut args = line.split(" ");
        match args.next() {
            Some("v") => {
                vertices.push(Vec3::new(
                    args.next().unwrap().parse::<f32>().unwrap(),
                    args.next().unwrap().parse::<f32>().unwrap(),
                    args.next().unwrap().parse::<f32>().unwrap(),
                ));
            }
            Some("vt") => {
                texture_coordinates.push(Vec2::new(
                    args.next().unwrap().parse::<f32>().unwrap(),
                    1.0 - args.next().unwrap().parse::<f32>().unwrap(),
                ));
            }
            Some("f") => {
                let face_resolver = |raw: &str| -> Vertex {
                    let mut iter = raw.split("/");
                    let vertex = iter.next().unwrap().parse::<usize>().unwrap() - 1;
                    let texture_coordinate = iter.next().unwrap().parse::<usize>().unwrap() - 1;
                    Vertex {
                        position: vertices.get(vertex).unwrap().clone(),
                        texture_coordinate: texture_coordinates
                            .get(texture_coordinate)
                            .unwrap()
                            .clone(),
                    }
                };
                let face: [Vertex; 3] = [
                    face_resolver(args.next().unwrap()),
                    face_resolver(args.next().unwrap()),
                    face_resolver(args.next().unwrap()),
                ];
                faces.push(face);
            }
            _ => {}
        }
    }
    Ok(Mesh { triangles: faces })
}

pub fn load_texture(file_path: &str) -> u32 {
    unsafe {
        let img = ImageReader::open(file_path)
            .unwrap()
            .decode()
            .unwrap();
        let img_rgb = img.to_rgb8().as_raw().to_owned();
        let mut texture_id = 0;
        gl::GenTextures(1, &mut texture_id);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        debug_assert_ne!(texture_id, 0);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            img_rgb.as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        texture_id
    }
}
