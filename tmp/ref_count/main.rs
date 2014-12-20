//#![allow(unstable, experimental, deprecated, unused_variables)]


use entity::{ Entity, EntityKind };
use common::{ Location };
use world::{ World, Entities };
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
//use std::num::FloatMath;
//use std::f32;
//use std::collections::HashMap;

mod entity;
mod common;
mod creature;
mod world;


fn main() {
	let mut world: World = World::new();

	//let mut entities = Entities::new();

	let mut worm =  Rc::new(Entity::new("worm".to_string(), EntityKind::Creature, Location::origin()));
	let mut food = Rc::new(Entity::new("food".to_string(), EntityKind::Food, Location::new(50f32, 50f32)));
	let mut poison = Rc::new(Entity::new("poison".to_string(), EntityKind::Poison, Location::new(-50f32, -50f32)));

	

	world.entities().add(worm.clone());
	world.entities().add(food.clone());
	world.entities().add(poison.clone());

	world.entities().print();


	//worm.crawl((3f32 / 2f32) * common::PI, 10f32);
	//entity::print_entity(entities.get("worm"));

	//let worm_sniff = entity::sniff_loc(entities.get("worm").loc(), &entities);

	//println!("*** Worm Sniff: (sweet:{}) (sour:{}) ***", worm_sniff.sweet, worm_sniff.sour);



	//entity::sniff_loc(&Location::new(0f32, 0f32), &entities);


	/*
	let worm = entities.get_mut("worm").unwrap();
	worm.crawl((3f32 / 2f32) * common::PI, 10f32);
	entity::print_entity(worm);

	common::print_sniffs(entity::sniff_location(&worm.location, &entities));
	*/


	/*
	let mut worm = Entity::new("worm", EntityKind::Creature, Location::origin());
	let food = Entity::new("food", EntityKind::Food, Location { x: 50f32, y: 0f32 });
	let poison = Entity::new("poison", EntityKind::Poison, Location { x: -50f32, y: -0f32 });

	entities.insert("worm", &worm);
	entities.insert("food", &food);
	entities.insert("poison", &poison);
	*/

	/*
	world.add_entity("worm", entity::EntityKind::Creature, common::Location::origin());
	world.add_entity("food", entity::EntityKind::Food, common::Location { x: 50f32, y: 0f32 });
	world.add_entity("poison", entity::EntityKind::Poison, common::Location { x: -50f32, y: -0f32 });

	entities.insert("worm", box Entity::new("worm", EntityKind::Creature, Location::origin()));
	entities.insert("food", box Entity::new("food", EntityKind::Food, Location { x: 50f32, y: 0f32 }));
	entities.insert("poison", box Entity::new("poison", EntityKind::Poison, Location { x: -50f32, y: -0f32 }));
	

	world::print_world(&mut world);
	
	world.entity("worm").crawl((3f32 / 2f32) * common::PI, 10f32);
	entity::print_entity(world.entity("worm"));

	let worm_sniff = world.sniff("worm");
	common::print_sniffs(&worm_sniff);
	*/


	//worm_sniff = world.entity("worm").sniff(world);
	//common::print_sniffs(&worm_sniff);
	//common::print_sniffs(entity::sniff_loc(&entities.get("worm").loc, &entities));

}

/*
pub fn add_entity<'a>(&'a mut self, name: &'static str, kind: EntityKind, position: Location) -> &mut Box<Entity> {
		let mut new_ent = box Entity::new(name, kind);
		new_ent.location = position;
		let name: &'static str = new_ent.name;
		self.entities.insert(name, new_ent);
		self.entity(name)
	}
*/

// * f32::consts::PI
