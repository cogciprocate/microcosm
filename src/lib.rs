
use entity::{ EntityBody, EntityKind, EntityBrain, Mobile };
use common::{ Location, Scent };
use world::{ World };
use std::option::{ Option };
//use std::rc::Rc;
//use std::rc::Weak;
//use std::cell::RefCell;
//use std::num::Float;
//use std::num::FloatMath;

pub mod entity;
pub mod common;
pub mod world;



fn run_test() {
	let mut world: World = World::new();

	let worm =  EntityBody::new("worm".to_string(), EntityKind::Creature, Location::origin());
	let food = EntityBody::new("food".to_string(), EntityKind::Food, Location::new(50f32, 50f32));
	let poison = EntityBody::new("poison".to_string(), EntityKind::Poison, Location::new(-100f32, -50f32));


	world.entities().add(worm);
	world.entities().add(food);
	world.entities().add(poison);
	world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(150f32, -200f32)));
	world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(-150f32, -250f32)));
	world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(550f32, -200f32)));
	world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(-1150f32, -250f32)));

	world.entities().print();


	let mut worm_brain = EntityBrain::new(world.sniff_at_entity(0));

	for i in range(0u, 100000) {

		if worm_brain.act(&mut world) == Option::None {
			println!("Everything eaten after {} iterations.", i);
			break
		}
		//worm_brain.print();
	}

	worm_brain.print();
	world.entities().print();

	//

}

