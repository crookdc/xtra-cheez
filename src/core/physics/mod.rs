use crate::core::ecs::component::Transform;
use crate::core::ecs::{Query, ECS};
use glam::Vec3;

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
        transform.position.y = 0.0;
        let mut body = ecs.clone_component::<DynamicPhysicsBody>(id).unwrap();
        for other in statics.iter() {
            let other = other.clone();
            let mut other_transform = ecs.clone_component::<Transform>(other).unwrap();
            other_transform.position.y = 0.0;
            let other_body = ecs.clone_component::<PhysicsBody>(other).unwrap();

            let distance = transform.distance(other_transform);
            if distance > body.base.radius + other_body.radius {
                continue;
            }
            let mut impulse = -body.force - (body.velocity * other_body.mass);
            // Adds some extra impulse to the inverted forward vector, otherwise the bodies gets
            // stuck to each other after colliding.
            impulse += transform.forward() * -5.0;
            ecs.get_component::<DynamicPhysicsBody>(id)
                .unwrap()
                .borrow_mut()
                .downcast_mut::<DynamicPhysicsBody>()
                .unwrap()
                .force += impulse;
            println!("{:?}", impulse);
            break;
        }
    }
}

#[derive(Clone)]
pub struct PhysicsBody {
    pub mass: f32,
    pub radius: f32,
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
