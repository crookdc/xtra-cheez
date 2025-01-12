use crate::core::ecs::component::Transform;
use crate::core::ecs::{Query, ECS};
use crate::core::radians;
use glam::{Mat4, Vec2, Vec3};

pub fn collision_system(ecs: &mut ECS, delta_time: f32) {
    let dynamic = ecs.query(
        &Query::new()
            .with::<Transform>()
            .with::<DynamicPhysicsBody>()
            .build(),
    );
    let statics = ecs.query(
        &Query::new()
            .with::<Transform>()
            .with::<PhysicsBody>()
            .build(),
    );
    // The real requirement for the collision checker as it stands is to be able to detect
    // collisions between static and dynamic objects, not between two dynamic objects, since there
    // will only be one dynamic object on a given level (the player). This is one of those parts of
    // this program that I will happily revisit once I have my PoC completed.
    for id in dynamic {
        let mut transform = ecs.clone_component::<Transform>(id).unwrap();
        let mut body = ecs.clone_component::<DynamicPhysicsBody>(id).unwrap();
        let dynamic_bounds = get_bounds(&transform, body.base.width, body.base.depth);

        for other in statics.iter() {
            let other = other.clone();
            let other_body = ecs.clone_component::<PhysicsBody>(other).unwrap();
            let static_bounds = get_bounds(
                &ecs.clone_component::<Transform>(other).unwrap(),
                other_body.width,
                other_body.depth,
            );
            // Figure out if the two are overlapping. Since we do not deal with the Y-axis in this
            // project, the collision detection will work on a 2D plane. In a 3D world this will
            // effectively mean that all objects have unbounded height
            if is_intersecting(dynamic_bounds, static_bounds) {
                let mut impulse = -body.force - (body.velocity * other_body.mass);
                // Adds some extra impulse to the inverted forward vector, otherwise the bodies gets
                // stuck to each other after colliding.
                impulse += transform.forward() * -2.0;
                ecs.get_component::<DynamicPhysicsBody>(id)
                    .unwrap()
                    .borrow_mut()
                    .downcast_mut::<DynamicPhysicsBody>()
                    .unwrap()
                    .force += impulse;
                break;
            }
        }
    }
}

fn is_intersecting(a: (Vec2, Vec2), b: (Vec2, Vec2)) -> bool {
    a.0.x >= b.1.x && a.1.x <= b.0.x && a.0.y >= b.1.y && a.1.y <= b.0.y
}

fn get_bounds(transform: &Transform, width: f32, depth: f32) -> (Vec2, Vec2) {
    let matrix = Mat4::from_translation(transform.position)
        * Mat4::from_scale(transform.scale)
        * Mat4::from_rotation_y(radians(transform.rotation.y))
        * Mat4::from_scale(Vec3::new(width, 1.0, depth));
    let top_right = matrix.transform_point3(Vec3::new(0.5, 0.0, 0.5));
    let bottom_left = matrix.transform_point3(Vec3::new(-0.5, 0.0, -0.5));
    (
        Vec2::new(top_right.x, top_right.z),
        Vec2::new(bottom_left.x, bottom_left.z),
    )
}

#[derive(Clone)]
pub struct PhysicsBody {
    pub mass: f32,
    pub width: f32,
    pub depth: f32,
}

#[derive(Clone)]
pub struct DynamicPhysicsBody {
    pub base: PhysicsBody,
    pub force: Vec3,
    pub velocity: Vec3,
}

pub fn velocity_system(ecs: &mut ECS, delta_time: f32) {
    let bodies = ecs.query(
        &Query::new()
            .with::<Transform>()
            .with::<DynamicPhysicsBody>()
            .build(),
    );
    for id in bodies {
        ecs.update_component::<DynamicPhysicsBody>(id, &mut |mut body| {
            body.velocity = (body.force / body.base.mass) * delta_time;
            body.force -= body.velocity * 4.0;
            body
        })
        .unwrap();

        let body = ecs.clone_component::<DynamicPhysicsBody>(id).unwrap();
        ecs.update_component::<Transform>(id, &mut |mut transform| {
            transform.position += body.velocity;
            transform
        })
        .unwrap();
    }
}
