use psx::core::ecs::world::World;

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

#[test]
fn delete_resource() {
    let mut world = World::new();
    world.create_resource(FpsResource(60));
    {
        let fps = world.get_resource::<FpsResource>().unwrap();
        assert_eq!(fps.0, 60);
    }
    world.delete_resource::<FpsResource>();
    let fps = world.get_resource::<FpsResource>();
    assert!(fps.is_none());
}

struct FpsResource(pub u32);
