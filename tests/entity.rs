use psx::core::ecs::world::World;
use psx::core::ecs::Query;

#[test]
fn query_entities() {
    struct Location();
    struct Speed();

    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Speed>();

    let entity = world.create_entity();
    world.attach_entity_component(entity, Location()).unwrap();
    world.attach_entity_component(entity, Speed()).unwrap();

    let query = Query::new().with::<Location>().with::<Speed>().build();
    let result = world.resolve(&query);
    assert_eq!(1, result.len());
    assert!(world.get_entity_component::<Location>(result[0]).is_some());
    assert!(world.get_entity_component::<Speed>(result[0]).is_some());
}
