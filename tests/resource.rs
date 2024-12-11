use psx::core::ecs::World;

#[test]
fn get_resource_immutable() {
    let mut world = World::new();
    world.create_resource(FpsResource(60));

    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 60);
}

#[test]
fn get_resource_mutable() {
    let mut world = World::new();
    world.create_resource(FpsResource(60));
    {
        let fps: &mut FpsResource = world.get_resource_mut::<FpsResource>().unwrap();
        fps.0 += 1;
    }
    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 61);
}

struct FpsResource(pub u32);