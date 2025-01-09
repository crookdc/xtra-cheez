use glam::{Vec2, Vec3};
use image::ImageReader;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub struct MeshLoader(HashMap<String, Mesh>);

impl MeshLoader {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn load_obj_file(&mut self, file_path: &str) -> Result<Mesh, io::Error> {
        let key = String::from(file_path);
        if self.0.contains_key(&key) {
            return Ok(self.0.get(&key).unwrap().clone());
        }
        let mesh = parse_obj_file(file_path)?;
        self.0.insert(key, mesh.clone());
        Ok(mesh)
    }
}

#[derive(Clone)]
pub struct Mesh {
    pub materials: Vec<Material>,
    faces: Vec<Face>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            materials: vec![],
            faces: vec![],
        }
    }
}

impl Mesh {
    pub fn serialize(&self) -> Vec<f32> {
        let mut buffer = vec![];
        for face in &self.faces {
            for vertex in face {
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
        (self.faces.len() * 3) as u32
    }
}

type Face = [Vertex; 3];

#[derive(Default, Clone)]
pub struct Vertex {
    position: Vec3,
    texture_coordinate: Vec2,
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub first_index: i32,
    pub count: i32,
    pub texture_id: Option<u32>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            first_index: 0,
            count: 0,
            texture_id: None,
        }
    }
}

pub fn parse_obj_file(file_path: &str) -> Result<Mesh, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut v: Vec<Vec3> = vec![];
    let mut vt: Vec<Vec2> = vec![];
    let mut faces: Vec<Face> = vec![];

    let mut matlib: HashMap<String, Option<u32>> = HashMap::new();
    let mut materials: Vec<Material> = vec![];

    let mut vertex_index = 0;
    for line in reader.lines() {
        let line = line?;
        let mut args = line.split(" ");
        match args.next() {
            Some("mtllib") => {
                matlib =
                    parse_mtl_file(("assets/models/".to_owned() + args.next().unwrap()).as_ref())?;
            }
            Some("v") => {
                v.push(Vec3::new(
                    args.next().unwrap().parse::<f32>().unwrap(),
                    args.next().unwrap().parse::<f32>().unwrap(),
                    args.next().unwrap().parse::<f32>().unwrap(),
                ));
            }
            Some("vt") => {
                vt.push(Vec2::new(
                    args.next().unwrap().parse::<f32>().unwrap(),
                    1.0 - args.next().unwrap().parse::<f32>().unwrap(),
                ));
            }
            Some("f") => {
                let parse_vertex = |raw: &str| -> Vertex {
                    let mut iter = raw.split("/");
                    let vertex = iter.next().unwrap().parse::<usize>().unwrap() - 1;
                    let texture_coordinate = iter.next().unwrap().parse::<usize>().unwrap() - 1;
                    Vertex {
                        position: v.get(vertex).unwrap().clone(),
                        texture_coordinate: vt.get(texture_coordinate).unwrap().clone(),
                    }
                };
                faces.push([
                    parse_vertex(args.next().unwrap()),
                    parse_vertex(args.next().unwrap()),
                    parse_vertex(args.next().unwrap()),
                ]);
                vertex_index += 3;
                materials.last_mut().unwrap().count += 3;
            }
            Some("usemtl") => {
                let name = String::from(args.next().unwrap());
                materials.push(Material {
                    first_index: vertex_index,
                    count: 0,
                    texture_id: matlib.get(&name).unwrap().clone(),
                });
            }
            _ => {}
        }
    }
    Ok(Mesh { faces, materials })
}

fn parse_mtl_file(file_path: &str) -> Result<HashMap<String, Option<u32>>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut matlib = HashMap::new();
    let mut current: Option<String> = None;
    for line in reader.lines() {
        let line = line?;
        let mut args = line.split(" ");
        match args.next() {
            Some("newmtl") => {
                current = Some(String::from(args.next().unwrap()));
                matlib.insert(current.clone().unwrap(), None);
            }
            Some("map_Kd") => {
                let texture_file_path =
                    "assets/models/materials/textures/".to_owned() + args.next().unwrap();
                matlib.insert(
                    current.clone().unwrap(),
                    Some(load_gl_texture(texture_file_path.as_ref())),
                );
            }
            _ => {}
        }
    }
    Ok(matlib)
}

pub fn load_gl_texture(file_path: &str) -> u32 {
    unsafe {
        let mut texture_id = 0;
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let img = ImageReader::open(file_path).unwrap().decode().unwrap();
        let img_rgb = img.to_rgba8().as_raw().to_owned();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img_rgb.as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::BindTexture(gl::TEXTURE_2D, 0);
        texture_id
    }
}
