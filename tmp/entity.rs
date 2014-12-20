

use common;
use common::{ Location, Scent };
//use world::{ World, Entities };
use std::num::FloatMath;
use std::num::Float;
use std::collections::HashMap;
//use std::collections::hash_map;

//pub use self::EntityKind::{ Food, Poison, Creature };

/*
pub struct EntityCore {
	name: String,
	loc: Location,
	kind: EntityKind,

}

impl EntityCore {
	pub fn new(name: String, kind: EntityKind, loc: Location) -> EntityCore {
		EntityCore {
			name: name,
			loc: loc,
			kind: kind,
		//last_scent: Scent::none(),
		}
	}
}
*/

pub trait Worldly {
	fn scent(&self) -> Scent;
	fn loc(&self) -> &Location;
	fn name(&self) -> &String;
	fn kind(&self) -> &EntityKind;
}


pub struct Entity {
	//core: EntityCore,

	
	name: String,
	loc: Location,
	kind: EntityKind,
	

	//pub last_scent: Scent,
}
impl Entity {

	pub fn new(name: String, kind: EntityKind, loc: Location) -> Entity {
		Entity {
			name: name,
			loc: loc,
			kind: kind,
		}
		/*
		Entity { 
			core: EntityCore::new(name, kind, loc) 
		}
		*/
	}

	pub fn print(&self) {
		println!("[Entity: {}]:  (loc: {}, {})  (kind: {}) ",
				 self.name(), 
				 self.loc().x, 
				 self.loc().y, 
				 entity_kind_string(self.kind()),
				 //entity.last_scent.sweet,
				 //entity.last_scent.sour,
		 );
	}

	pub fn crawl(&mut self, direction: f32, magnitude: f32) {
		// direction is in radians where up (north) is pi/2 and right (east) is 0

		let x = direction.cos() * magnitude;
		let y = direction.sin() * magnitude;

		self.loc.x += x;
		self.loc.y += y;

	}
}

impl Worldly for Entity {
	fn scent(&self) -> Scent {
		match self.kind {
			EntityKind::Food =>	Scent { sweet: 100f32, sour: 0f32 },
			EntityKind::Poison => Scent::new(0f32, 100f32),
			EntityKind::Creature =>	Scent::new(0f32, 0f32),
		}
	}

	fn loc(&self) -> &Location {
		&self.loc
	}

	fn name(&self) -> &String {
		&self.name
	}

	fn kind(&self) -> &EntityKind {
		&self.kind
	}

	/*
	pub fn sniff(&mut self, entity_name: String) -> Vec<Sniff> {
		let entity = self.entity(entity_name);
		println!("== World::sniff() == [entity_loc]: ({}, {})", entity.loc.x, entity.loc.y);

		let mut new_sniff = Vec::new();
		new_sniff.push(Sniff::new(Scent::new(55f32, 0f32), 100f32, PI));
		new_sniff.push(Sniff::new(Scent::new(0f32, 55f32), -100f32, PI / 2f32));
		new_sniff
	}
	*/

}

pub enum EntityKind {
	Food,
	Poison,
	Creature,
}

pub fn print_entity(entity: &Entity) {
	println!("[Entity: {}]:  (loc: {}, {})  (kind: {}) ",
			 entity.name(), 
			 entity.loc().x, 
			 entity.loc().y, 
			 entity_kind_string(entity.kind()),
			 //entity.last_scent.sweet,
			 //entity.last_scent.sour,
	 );
}

pub fn entity_kind_string(entity: &EntityKind) -> &'static str {
	match entity {
		&EntityKind::Food 			=> "Food",
		&EntityKind::Poison 		=> "Poison",
		&EntityKind::Creature 		=> "Creature",
	}
}

pub fn distance(loc_a: &Location, loc_b: &Location) -> f32 {
    let x_delta = loc_b.x - loc_a.x;
    let y_delta = loc_b.y - loc_a.y;
    let dist = (x_delta.powi(2i32) + y_delta.powi(2i32)).sqrt();

    //println!("*** x_delta: {}; x_delta.abs(): {} ***", x_delta, x_delta.abs());
    //println!("*** y_delta: {}; y_delta.abs(): {} ***", y_delta, y_delta.abs());
    //println!("*** distance: {} ***", dist);

    dist
}


/*
pub fn print_entities<'a>(entities: &Entities) {
	// for val in world.entities.values() {
	// for key in world.entities.keys() {

	println!("entity::print_entities:");

	for (key, &val) in entities.iter() {
		print!("	");
		print!("{}: ", key);
		print_entity(val);
	}
}
*/


/*
pub fn print_entity(entity: &Box<Entity>) {
	println!("[Entity]:  (name: {})  (loc: {}, {})  (kind: {})  (last_scent: sweet:{}, sour:{}) ",
			 entity.name, 
			 entity.loc.x, 
			 entity.loc.y, 
			 entity_kind_string(&entity.kind),
			 entity.last_scent.sweet,
			 entity.last_scent.sour,
	 );
}

pub fn print_entities<'a>(entities: HashMap<String, Box<Entity<'a>>>) {
	// for val in world.entities.values() {
	// for key in world.entities.keys() {

	for (key, val) in entities.iter() {
		print!("	");
		print!("{}: ", key);
		let mut ent: &Box<Entity> = val;

		print_entity(ent);
	}
}
*/


/*
pub type Entities<'a> = HashMap<String, Box<Entity<'a>>>;

trait Insertable<'a> {
    fn add(&self, entity: &'a Entity);
}

impl <'a> Insertable <'a> for Entities<'a> {
	fn add (&self, entity: &Entity) {
			// Insert some shit!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! ===========****************************
	}
}


pub type Entities<'a> = HashMap<String, Box<Entity<'a>>>;

impl <'a> Entities<'a> {
	fn new<'a> () -> HashMap<String, Box<Entity<'a>>> {
		HashMap::new()
	}
}
*/
