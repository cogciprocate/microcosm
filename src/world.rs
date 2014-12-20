
//use entity;
use entity::{ EntityBody, EntityKind, Worldly };
use common;
use common::{ Scent };
use std::num::Float;
//use std::iter::Iterator;
//use std::num::FloatMath;
//use std::clone::Clone;
//use std::rc::Rc;
//use std::rc::Weak;
//use std::cell::RefCell;
//use std::cell::RefMut;

//mod entity;
//mod common;

pub struct World<'a> {
	entities: Entities<'a>,

}
impl <'a>World<'a> {
	pub fn new() -> World<'a> {
		World { 
			entities: Entities::new(),
		}
	}

	pub fn entities(&mut self) -> &mut Entities {
		&mut self.entities
	}

	pub fn sniff_at_entity(&self, ent_idx: uint) -> Scent {
		let ent = self.entities.get(ent_idx);
		let ent_loc = ent.loc();
		let ent_uid = ent.uid;
		drop(ent);

		let mut loc_scent: Scent = Scent::none();
		//println!("======== Entity::sniff_loc() == [entity_loc]: ({}, {}) ========", loc.x, loc.y);

		for e in self.entities.entities.iter().filter(|e| ent_filter(*e, ent_uid)) {
			//if ent.eaten == true { continue };
			let mut scent = e.scent();
			let dist: f32 = common::distance_between(&e.loc(), &ent_loc);
			let inten = if common::floats_eq(0f32, dist) { 1f32 } else { 1f32 / dist.powi(2) };

			scent.scale(inten);
			loc_scent.add(scent);
		}

		loc_scent
	}

	pub fn feed_entity(&mut self, ent_idx: uint) -> EntityKind {
		//let entity = self.entities.get(ent_idx);
		let ent_loc = self.entities.get(ent_idx).loc();
		let ent_uid = self.entities.get(ent_idx).uid;

		for e in self.entities.entities.iter_mut().filter(|e| ent_filter(&**e, ent_uid)) {
			//if ent.eaten == true { continue };
			let dist: f32 = common::distance_between(&e.loc(), &ent_loc);
			if dist <= 1f32 {
				println!("Eating a {} at {}", e.kind(), &e.loc())
				e.eaten = true;
				return e.kind()
			}
		}
		EntityKind::None
	}
}

fn ent_filter(e: &EntityBody, ent_uid: uint) -> bool {
	e.eaten == false && e.uid != ent_uid
}


pub struct Entities<'a> {
	entities: Vec<EntityBody>,
}
impl <'a> Entities<'a> {
	pub fn new<'a>() -> Entities<'a> {
		Entities { entities: Vec::new() }
	}

	pub fn add(&mut self, mut entity: EntityBody) {
		println!("Adding EntityBody: ");
		entity.uid = self.entities.len();
		self.entities.push(entity);
	}

	pub fn get(&self, idx: uint) -> &EntityBody {
		&self.entities[idx]
	}

	pub fn get_mut(&mut self, idx: uint) -> &mut EntityBody {
		&mut self.entities[idx]
	}

	pub fn print(&self) {

		println!("");
		println!("Printing {} Entities... ", self.entities.len());

		for ent in self.entities.iter() {
			ent.print();
		}
		println!("");
	}

	/*
	pub fn iter(&self) -> vec::Entries<EntityBody> {
		self.iter()
	}
	*/
	
}

