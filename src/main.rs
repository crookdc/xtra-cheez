use psx::core::ecs::world::World;
use psx::core::ecs::Query;
use psx::core::Engine;
use sdl2::rect::FRect;

struct Position {
    x: f32,
    y: f32,
}
struct Rectangle(f32, f32);

fn main() {
    let mut sdl = sdl2::init().unwrap();
    let mut engine = Engine::new(&mut sdl);
    engine.register_renderer(rectangle_renderer);
    engine.register_system(movement_system);

    engine.get_world().register_component::<Position>();
    engine.get_world().register_component::<Rectangle>();

    let paddle = engine.get_world().create_entity();
    engine
        .get_world()
        .attach_entity_component(paddle, Position { x: 0.0, y: 0.0 })
        .unwrap();
    engine
        .get_world()
        .attach_entity_component(paddle, Rectangle(20.0, 80.0))
        .unwrap();

    engine.run();
}

fn movement_system(world: &mut World) {
    let entities = world.resolve(&Query::new().with::<Position>().build());
    entities.iter().for_each(|entity| {
        let position = world.get_entity_component::<Position>(*entity).unwrap();
        position.borrow_mut().downcast_mut::<Position>().unwrap().y += 0.5;
    });
}

fn rectangle_renderer(world: &mut World, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let entities = world.resolve(&Query::new().with::<Rectangle>().with::<Position>().build());
    entities.iter().for_each(|entity| {
        let position = world.get_entity_component::<Position>(*entity).unwrap();
        let rectangle = world.get_entity_component::<Rectangle>(*entity).unwrap();
        canvas
            .draw_frect(FRect::new(
                position.borrow().downcast_ref::<Position>().unwrap().x,
                position.borrow().downcast_ref::<Position>().unwrap().y,
                rectangle.borrow().downcast_ref::<Rectangle>().unwrap().0,
                rectangle.borrow().downcast_ref::<Rectangle>().unwrap().1,
            ))
            .unwrap();
    });
}
