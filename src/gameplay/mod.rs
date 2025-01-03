use crate::core::ecs::component::{Material, Model, Transform};
use crate::core::ecs::ECS;
use crate::core::render::model::parse_obj_file;
use crate::core::render::shader::Shader;
use glam::Vec3;
use std::collections::HashSet;

const TILE_SIZE: f32 = 20.0;

type Level = Vec<Vec<HashSet<EntityType>>>;

#[derive(Eq, Hash, PartialEq)]
enum EntityType {
    Structure,
    Decoration,
    Objective,
}

/**
Generation of a cityscape of width * height size. The units of `width` and `height` are simply the
number of possible structures, not any actual unit of measurement found in the real world. The
generated city is composed of equally sized randomized 3D structures in a simple grid with blocking
structures occurring on some streets. The generation algorithm should guarantee that there exists no
unreachable space on the map by checking if a blocker has already been placed in a neighborhood, if
so then another block can only be placed in such a way that the resulting two blockers share a wall.
*/
pub fn generate_cityscape(ecs: &mut ECS, width: usize, height: usize) {
    let mut level = Level::with_capacity(width);
    for i in 0..width {
        level.insert(i, Vec::new());
        for j in 0..height {
            level[i].insert(j, HashSet::new());
        }
    }
    generate_structure_pass(ecs, &mut level);
    // generate_decoration_pass(ecs, &mut level);
    // generate_objective_pass(ecs, &mut level);
    // generate_road_pass(ecs, &mut level);
}

fn generate_structure_pass(ecs: &mut ECS, level: &mut Level) {
    for i in 0..level.len() {
        if i % 2 == 0 {
            // Every other row should be left empty as road in the city for the player to use
            continue;
        }
        for j in 0..level[i].len() {
            level[i].push(HashSet::new());
            if j % 5 == 0 {
                // Every fifth column should be left empty as road in the city for the player to use
                continue;
            }
            spawn_structure(ecs, j as f32, i as f32);
            level[i][j].insert(EntityType::Structure); // Structure was added to the tile (i,j)
        }
    }
}

fn spawn_structure(ecs: &mut ECS, tile_x: f32, tile_y: f32) {
    let model = Model::new(
        Material {
            shader_id: ecs.get_resource::<Shader>().unwrap().get_id(),
            texture_id: None,
        },
        &parse_obj_file("assets/models/tile.obj").unwrap(),
    );
    let id = ecs.create_entity();
    ecs.attach_component(
        id,
        Transform {
            scale: Vec3::new(10.0, 150.0, 10.0),
            position: Vec3::new(TILE_SIZE * tile_x, 9.4, TILE_SIZE * tile_y),
            rotation: Vec3::default(),
            pivot: Vec3::default(),
        },
    )
    .unwrap();
    ecs.attach_component(id, model).unwrap();
}
