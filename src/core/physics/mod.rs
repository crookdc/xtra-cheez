use crate::core::ecs::component::{PhysicsBody, Transform};
use crate::core::ecs::{Query, ECS};

pub fn system(ecs: &mut ECS, delta_time: f32) {
    let bodies = ecs.query(
        &Query::new()
            .with::<Transform>()
            .with::<PhysicsBody>()
            .build(),
    );
    for id in bodies {
        ecs.update_component::<PhysicsBody>(id, &mut |mut body| {
            body.velocity = (body.force / body.mass) * delta_time;
            body.force -= body.velocity * 4.0;
            body
        })
        .unwrap();

        let body = ecs.clone_component::<PhysicsBody>(id).unwrap();
        ecs.update_component::<Transform>(id, &mut |mut transform| {
            transform.position += body.velocity;
            transform
        })
        .unwrap();
    }
}
