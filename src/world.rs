
//use entity;
use entity::{ EntityBody, EntityKind, Worldly, Mobile };
use common;
use common::{ Scent, Peek };
use num::Float;
//use std::iter::{ Iterator };
//use std;
//use std::iter::Iterator;
//use std::num::FloatMath;
//use std::clone::Clone;
//use std::rc::Rc;
//use std::rc::Weak;
//use std::cell::RefCell;
//use std::cell::RefMut;

//mod entity;
//mod common;

pub struct World{
	entities: Entities,
	width: u32,
}
impl World {
	pub fn new(width: u32) -> World {
		World { 
			entities: Entities::new(),
			width: width,
		}
	}

	pub fn entities(&mut self) -> &mut Entities {
		&mut self.entities
	}

	pub fn peek_from(&self, ent_idx: usize) -> Box<Peek> {
		let ent = self.entities.get(ent_idx);
		let ent_loc = ent.loc();
		let ent_uid = ent.uid;
		let ent_head = ent.heading();
		drop(ent);

		let mut peek = Box::new(Peek::new(self.width));

		for e in self.entities.entities.iter().filter(|e| ent_filter(*e, ent_uid)) {
			let bear = common::bearing(&e.loc(), &ent_loc) + ent_head;
			let dist = common::distance(&e.loc(), &ent_loc);
			let vis_size: u32 = common::vis_size(dist, self.width);
			peek.render_ent(bear, vis_size, dist);
			
			//println!("Entity:{} -- Bearing:{}, vis_dia:{}, peek.len():{}", e.uid, bear, vis_size, peek.peek.len());

		}
		/*
		println!("");
		for p in peek.peek.iter() {
			print!("{}", p);
		}
		println!("")
		*/
		peek

	}

	pub fn sniff_from(&self, ent_idx: usize) -> Scent {
		let ent = self.entities.get(ent_idx);
		let ent_loc = ent.loc();
		let ent_uid = ent.uid;
		drop(ent);

		let mut loc_scent: Scent = Scent::none();
		//println!("======== Entity::sniff_loc() == [entity_loc]: ({}, {}) ========", loc.x, loc.y);

		for e in self.entities.entities.iter().filter(|e| ent_filter(*e, ent_uid)) {
			//if ent.eaten == true { continue };
			let mut scent = e.scent();
			let dist = common::distance(&e.loc(), &ent_loc);

			let inten = if common::floats_eq(0f32, dist) { 1f32 / 0.0001f32.powi(2) } else { 1f32 / dist.powi(2) };
			let inten = (ent_idx & 0xFF) as f32;

			scent.scale(inten);
			loc_scent.add(scent);
		}

		loc_scent
	}

	pub fn feed_entity(&mut self, ent_idx: usize) -> EntityKind {
		//let entity = self.entities.get(ent_idx);
		let ent_loc = self.entities.get(ent_idx).loc();
		let ent_uid = self.entities.get(ent_idx).uid;
		let ent_name = self.entities.get(ent_idx).name.clone();

		for e in self.entities.entities.iter_mut().filter(|e| ent_filter(&**e, ent_uid)) {
			//if ent.eaten == true { continue };
			let dist: f32 = common::distance_between(&e.loc(), &ent_loc);
			if dist <= 1f32 {
				print!(" [{} is Eating a {} at {}] ", ent_name, e.kind(), &e.loc());
				e.eaten = true;
				return e.kind()
			}
		}
		EntityKind::None
	}
}

fn ent_filter(e: &EntityBody, ent_uid: usize) -> bool {
	e.eaten == false && e.uid != ent_uid
}


pub struct Entities {
	entities: Vec<EntityBody>,
}
impl  Entities {
	pub fn new() -> Entities {
		Entities { entities: Vec::new() }
	}

	pub fn add(&mut self, mut entity: EntityBody) {
		//println!("Adding EntityBody: ");
		entity.uid = self.entities.len();
		self.entities.push(entity);
	}

	pub fn get(&self, idx: usize) -> &EntityBody {
		&self.entities[idx]
	}

	pub fn get_mut(&mut self, idx: usize) -> &mut EntityBody {
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
	
	
}

