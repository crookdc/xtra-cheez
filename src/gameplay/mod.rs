use crate::core::ecs::component::{CameraTarget, KeyboardControls, Model, PhysicsBody, Transform};
use crate::core::ecs::{Query, ECS};
use crate::core::render::model::MeshLoader;
use crate::core::Keymap;
use glam::Vec3;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

const OBSTACLE_MODEL_FILES: [&str; 3] = [
    "assets/models/building_07.obj",
    "assets/models/building_08.obj",
    "assets/models/building_09.obj",
];

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum EntityType {
    Obstacle,   // Any object causing a road block such as a structure or road maintenance
}

pub struct Maze(Vec<Vec<HashSet<EntityType>>>);

impl Maze {
    fn sized(width: usize, height: usize) -> Self {
        let mut s = Self(Vec::with_capacity(width));
        for i in 0..width {
            s.0.insert(i, Vec::new());
            for j in 0..height {
                s.0[i].insert(j, HashSet::new());
            }
        }
        s
    }

    fn fill(&mut self, entity_type: EntityType) {
        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                self.0[i][j].insert(entity_type);
            }
        }
    }

    fn clear(&mut self, tile: (usize, usize)) {
        self.0[tile.0][tile.1].clear();
    }

    fn neighbors(&self, tile: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        let mut neighbors = [None, None, None, None];
        // North neighbor
        if tile.1 > 0 && self.valid_tile((tile.0, tile.1 - 1)) {
            neighbors[0] = Some((tile.0, tile.1 - 1));
        }
        // South neighbor
        if self.valid_tile((tile.0, tile.1 + 1)) {
            neighbors[1] = Some((tile.0, tile.1 + 1));
        }
        // West neighbor
        if tile.0 > 0 && self.valid_tile((tile.0 - 1, tile.1)) {
            neighbors[2] = Some((tile.0 - 1, tile.1));
        }
        // East neighbor
        if self.valid_tile((tile.0 + 1, tile.1)) {
            neighbors[3] = Some((tile.0 + 1, tile.1));
        }
        neighbors
    }

    fn empty(&self, tile: (usize, usize)) -> bool {
        self.0[tile.0][tile.1].is_empty()
    }

    fn count_empty_neighbors(&self, tile: (usize, usize)) -> usize {
        self.neighbors(tile)
            .iter()
            .filter(|neighbor| {
                if let Some(n) = neighbor {
                    return self.empty(n.clone());
                }
                false
            })
            .count()
    }

    fn valid_tile(&self, tile: (usize, usize)) -> bool {
        tile.0 < self.0.len() && tile.1 < self.0[tile.0].len()
    }
}

pub fn generate_cityscape(width: usize, height: usize) -> Maze {
    let mut maze = Maze::sized(width, height);
    maze.fill(EntityType::Obstacle);
    generate_step(&mut maze, (0, 0));
    maze
}

fn generate_step(maze: &mut Maze, tile: (usize, usize)) {
    if maze.count_empty_neighbors(tile) >= 2 {
        return;
    }
    maze.clear(tile);
    let mut neighbors: Vec<(usize, usize)> = maze
        .neighbors(tile)
        .iter()
        .filter_map(|tile| {
            if let Some(tile) = tile {
                return if maze.empty(tile.clone()) {
                    None
                } else {
                    Some(tile.clone())
                };
            }
            None
        })
        .collect();
    while !neighbors.is_empty() {
        let selection = thread_rng().gen_range(0..neighbors.len());
        generate_step(maze, neighbors[selection]);
        neighbors.remove(selection);
    }
}

pub fn build_entities(ecs: &mut ECS, maze: &Maze) {
    for i in 0..maze.0.len() {
        for j in 0..maze.0[i].len() {
            if maze.0[i][j].contains(&EntityType::Obstacle) {
                spawn_obstacle_on_tile(ecs, (i, j));
            }
        }
    }
}

fn spawn_obstacle_on_tile(ecs: &mut ECS, tile: (usize, usize)) {
    let id = ecs.create_entity();
    let mesh = ecs
        .get_resource_mut::<MeshLoader>()
        .unwrap()
        .load_obj_file(
            OBSTACLE_MODEL_FILES
                .get(thread_rng().gen_range(0..OBSTACLE_MODEL_FILES.len()))
                .unwrap(),
        )
        .unwrap();
    ecs.attach_component(id, Model::new(mesh)).unwrap();
    ecs.attach_component(
        id,
        Transform {
            scale: Vec3::new(8.0, 8.0, 8.0),
            position: Vec3::new(16.0 * tile.0 as f32, 0.0, 16.0 * tile.1 as f32),
            rotation: Vec3::default(),
            pivot: Vec3::default(),
        },
    )
    .unwrap();
}

pub fn build_player(ecs: &mut ECS) {
    let id = ecs.create_entity();
    ecs.attach_component(
        id,
        Transform {
            position: Vec3::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            pivot: Vec3::new(0.0, 0.0, -2.5),
            rotation: Vec3::new(0.0, 0.0, 0.0),
        },
    )
    .unwrap();
    let mesh = ecs
        .get_resource_mut::<MeshLoader>()
        .unwrap()
        .load_obj_file("assets/models/player.obj")
        .unwrap();
    ecs.attach_component(id, Model::new(mesh)).unwrap();
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

pub fn player_movement_system(ecs: &mut ECS, delta_time: f32) {
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
            body.force += transform.forward() * 300.0 * drive_dir * delta_time;
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
        transform.rotation.y += 30.0 / velocity.length() * delta_time * steer_dir;
        transform
    })
    .unwrap()
}
