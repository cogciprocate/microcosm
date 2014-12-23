
use entity::{ EntityBody, EntityKind, EntityBrain, Mobile };
use worm::{ WormBrain };
use common::{ Location };
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
pub mod worm;



pub fn run_test() {
	let mut world: World = World::new();

	let worm =  EntityBody::new("worm".to_string(), EntityKind::Creature, Location::origin());
	let food = EntityBody::new("food".to_string(), EntityKind::Food, Location::new(50f32, 50f32));
	//let poison = EntityBody::new("poison".to_string(), EntityKind::Poison, Location::new(-100f32, -50f32));

	let worm_uid = worm.uid;


	world.entities().add(worm);
	world.entities().add(food);
	//world.entities().add(poison);
	//world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(150f32, -200f32)));
	//world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(-150f32, -250f32)));
	//world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(550f32, -200f32)));
	//world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(-1150f32, -250f32)));
	world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(0f32, 110f32)));
	world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(-50f32, 0f32)));
	world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(0f32, -50f32)));
	world.entities().add(EntityBody::new("food".to_string(), EntityKind::Food, Location::new(130f32, 0f32)));

	world.entities().print();

	//world.peek_from(worm_uid);

	world.entities().get_mut(worm_uid).turn(0.25f32);

	//world.peek_from(worm_uid);



	let mut worm_brain = EntityBrain::new(worm_uid, &world);

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

