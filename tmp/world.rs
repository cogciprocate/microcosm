
use entity;
use entity::{ Entity, EntityKind };
use common::{ Location, Scent, PI};
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::cell::RefMut;

pub struct World<'a> {
	entities: RefCell<Entities<'a>>,

}
impl <'a>World<'a> {
	pub fn new() -> World<'a> {
		World { 
			entities: RefCell::new(Entities::new()),
		}
	}

	pub fn entities(&mut self) -> RefMut<Entities> {
		self.entities.borrow_mut()
	}

	/*
	
	pub fn add_entity<'a>(&'a mut self, entity: Rc<Entity>) {

		self.entities.borrow_mut().push(entity.clone().downgrade());

		//self.entities.insert(name, box Entity::new(name, kind, location));
	}

	pub fn entity(&mut self, name: &'static str) -> &mut Box<Entity> {
		let entity = match self.entities.get_mut (name) {
			Some(x) => x,
			None => panic!(),
		};
		entity
	}

	pub fn sniff(&mut self, entity_name: &'static str) -> Vec<Sniff> {
		let entity = self.entity(entity_name);
		println!("== World::sniff() == [entity_loc]: ({}, {})", entity.location.x, entity.location.y);

		let mut new_sniff = Vec::new();
		new_sniff.push(Sniff::new(Scent::new(55f32, 0f32), 100f32, PI));
		new_sniff.push(Sniff::new(Scent::new(0f32, 55f32), -100f32, PI / 2f32));
		new_sniff
	}
	*/

	/*
	pub fn sniff_location(&mut self, location: Location) -> Vec<Sniff> {
		//let entity = self.entity(entity_name);
		println!("== World::sniff_location() == [entity_loc]: ({}, {})", location.x, location.y);

		let mut new_sniff = Vec::new();
		new_sniff.push(Sniff::new(Scent::new(55f32, 0f32), 100f32, PI));
		new_sniff.push(Sniff::new(Scent::new(0f32, 55f32), -100f32, PI / 2f32));
		new_sniff
	}
	*/

	/*
	pub fn print_entity(&mut self, entity: &'static str) {
		print_entity(self.entity(entity));
	}
	*/
}


pub struct Entities<'a> {
	entities: Vec<Weak<Entity>>,
}
impl <'a> Entities<'a> {
	pub fn new<'a>() -> Entities<'a> {
		Entities { entities: Vec::new() }
	}

	pub fn add(&mut self, entity: Rc<Entity>) {
		println!("Adding Entity: ");
		entity.clone().downgrade().upgrade().unwrap().print();
		self.entities.push(entity.downgrade());
	}

	/*
	pub fn get(&self, name: &'static str) -> &Entity {
		let entity = match self.entities.get(name) {
			Some(x) => x,
			None => panic!(),
		};
		entity
	}
	

	pub fn get_mut(&mut self, name: &'static str) -> &mut Entity {
		let entity = match self.entities.get_mut(name) {
			Some(x) => x,
			None => panic!(),
		};
		entity
	}
	*/

	pub fn print(&self) {

		println!("Printing {} Entities... ", self.entities.len());

		// for val in world.entities.values() {
		// for key in world.entities.keys() {
		for ent_ref in self.entities.iter() {
			let ent = match ent_ref.upgrade() {
				Some(x) => x,
				None => {
					println!("*** Entity no longer exists ***")
					continue
				},
			};

			
			ent.print();
		}
	}

	/*
	pub fn iter(&self) -> hash_map::Entries<String, &'a Entity> {
		self.iter()
	}
	*/
}

/*
pub fn sniff_loc(loc: &Location, ents: &Entities) -> Scent {
	let mut loc_scent: Scent = Scent::none();
	println!("======== Entity::sniff_loc() == [entity_loc]: ({}, {}) ========", loc.x, loc.y);


	// for val in world.entities.values() {
	// for key in world.entities.keys() {
	for (_, ent) in ents.entities.iter() {
		let mut scent = ent.scent();
		let dist: f32 = distance(ent.loc(), loc);
		let inten = if common::floats_eq(0f32, dist) { 1f32 } else { 1f32 / dist.powi(2) };

		scent.scale(inten);

		loc_scent.add(scent);

		//new_sniff.push(Sniff::new(scent, inten, 0f32));

		println!("*** sniff_loc: [{}]: (distance: {}), (intensity: {}) ", ent.name(), dist, inten);
	}


	//new_sniff.push(Sniff::new(Scent::new(55f32, 0f32), 100f32, PI));
	//new_sniff.push(Sniff::new(Scent::new(0f32, 55f32), -100f32, PI / 2f32));
	loc_scent
}
*/
