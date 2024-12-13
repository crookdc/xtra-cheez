use psx::core::ecs::world::World;
use psx::core::ecs::Query;
use psx::core::Engine;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::rect::FRect;
use sdl2::Sdl;

struct Position(f32, f32);
struct Velocity(f32, f32);
struct Rectangle(f32, f32);
struct MovementController {
    up: Scancode,
    down: Scancode,
}

fn main() {
    let mut sdl = sdl2::init().unwrap();
    let mut engine = Engine::new(&mut sdl);
    engine.register_renderer(rectangle_renderer);
    engine.register_system(physics);
    engine.register_input_handler(movement);

    engine.get_world().register_component::<Position>();
    engine.get_world().register_component::<Velocity>();
    engine.get_world().register_component::<Rectangle>();
    engine
        .get_world()
        .register_component::<MovementController>();

    let left_paddle = engine.get_world().create_entity();
    engine
        .get_world()
        .attach_entity_component(left_paddle, Position(0.0, 0.0))
        .unwrap();
    engine
        .get_world()
        .attach_entity_component(left_paddle, Rectangle(20.0, 80.0))
        .unwrap();
    engine
        .get_world()
        .attach_entity_component(left_paddle, Velocity(0.0, 0.0))
        .unwrap();
    engine
        .get_world()
        .attach_entity_component(
            left_paddle,
            MovementController {
                up: Scancode::W,
                down: Scancode::S,
            },
        )
        .unwrap();

    let right_paddle = engine.get_world().create_entity();
    engine
        .get_world()
        .attach_entity_component(right_paddle, Position(780.0, 0.0))
        .unwrap();
    engine
        .get_world()
        .attach_entity_component(right_paddle, Rectangle(20.0, 80.0))
        .unwrap();
    engine
        .get_world()
        .attach_entity_component(right_paddle, Velocity(0.0, 0.0))
        .unwrap();
    engine
        .get_world()
        .attach_entity_component(
            right_paddle,
            MovementController {
                up: Scancode::Up,
                down: Scancode::Down,
            },
        )
        .unwrap();

    engine.run();
}

fn movement(world: &mut World, event: &Event) {
    let entities = world.resolve(
        &Query::new()
            .with::<MovementController>()
            .with::<Velocity>()
            .build(),
    );
    entities.iter().for_each(|entity| {
        let movement_controller = world
            .get_entity_component::<MovementController>(*entity)
            .unwrap();
        let velocity = world.get_entity_component::<Velocity>(*entity).unwrap();
        match event {
            Event::KeyDown { scancode, .. } => {
                if scancode.unwrap()
                    == movement_controller
                        .borrow()
                        .downcast_ref::<MovementController>()
                        .unwrap()
                        .up
                {
                    velocity.borrow_mut().downcast_mut::<Velocity>().unwrap().1 = -100.0;
                } else if scancode.unwrap()
                    == movement_controller
                        .borrow()
                        .downcast_ref::<MovementController>()
                        .unwrap()
                        .down
                {
                    velocity.borrow_mut().downcast_mut::<Velocity>().unwrap().1 = 100.0;
                }
            }
            Event::KeyUp { scancode, .. } => {
                if scancode.unwrap()
                    == movement_controller
                        .borrow()
                        .downcast_ref::<MovementController>()
                        .unwrap()
                        .up
                    || scancode.unwrap()
                        == movement_controller
                            .borrow()
                            .downcast_ref::<MovementController>()
                            .unwrap()
                            .down
                {
                    velocity.borrow_mut().downcast_mut::<Velocity>().unwrap().1 = 0.0;
                }
            }
            _ => {}
        }
    });
}

fn physics(world: &mut World, delta_time: f32) {
    let entities = world.resolve(&Query::new().with::<Position>().with::<Velocity>().build());
    entities.iter().for_each(|entity| {
        let position = world.get_entity_component::<Position>(*entity).unwrap();
        let velocity = world.get_entity_component::<Velocity>(*entity).unwrap();
        position.borrow_mut().downcast_mut::<Position>().unwrap().0 +=
            velocity.borrow().downcast_ref::<Velocity>().unwrap().0 * delta_time;
        position.borrow_mut().downcast_mut::<Position>().unwrap().1 +=
            velocity.borrow().downcast_ref::<Velocity>().unwrap().1 * delta_time;
    });
}

fn rectangle_renderer(world: &mut World, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let entities = world.resolve(&Query::new().with::<Rectangle>().with::<Position>().build());
    entities.iter().for_each(|entity| {
        let position = world.get_entity_component::<Position>(*entity).unwrap();
        let rectangle = world.get_entity_component::<Rectangle>(*entity).unwrap();
        canvas
            .draw_frect(FRect::new(
                position.borrow().downcast_ref::<Position>().unwrap().0,
                position.borrow().downcast_ref::<Position>().unwrap().1,
                rectangle.borrow().downcast_ref::<Rectangle>().unwrap().0,
                rectangle.borrow().downcast_ref::<Rectangle>().unwrap().1,
            ))
            .unwrap();
    });
}
