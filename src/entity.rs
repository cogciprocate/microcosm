

use common;
use common::{ Location, Scent };
use world::{ World };
use std::num::FloatMath;
use std::fmt::{ Show, Formatter, Error };
//use std::num::Float;
//use std::collections::HashMap;

pub trait Worldly {
	fn scent(&self) -> Scent;
	fn loc(&self) -> Location;
	fn name(&self) -> &str;
	fn kind(&self) -> EntityKind;
}

pub trait Mobile {
	fn propel(&mut self);
	fn turn(&mut self, amt: f32);
	fn heading(&self) -> f32;
}

pub trait WormBrain {
	fn act(&mut self, world: &mut World) -> Option<()>;
	fn navigate(&mut self, world: &mut World, scent_new: Scent);
	fn eat(&mut self, world: &mut World, scent_new: &Scent);
	fn propel(&self, world: &mut World);
}


pub struct EntityBody {
	name: String,
	loc: Location,
	heading: f32,
	kind: EntityKind,
	pub eaten: bool,
	pub uid: uint,
}
impl EntityBody {
	pub fn new(name: String, kind: EntityKind, loc: Location) -> EntityBody {
		EntityBody {
			name: name,
			loc: loc,
			heading: 0f32,
			kind: kind,
			eaten: false,
			uid: 0,
		}

	}

	pub fn print(&self) {
		println!("{}", self);
	}

}
impl Show for EntityBody {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "[EntityBody: {}]: (uid: {}) (loc: {}, {})  (kind: {}) (eaten:{})",
				 self.name(),
				 self.uid,
				 self.loc().x, 
				 self.loc().y, 
				 self.kind,
				 self.eaten,
	 	) 
	}
}

impl Mobile for EntityBody {
	fn turn(&mut self, amt: f32) {
		self.heading += amt;
		self.heading = common::normalize_bearing(self.heading);
	}

	fn heading(&self) -> f32 {
		self.heading
	}

	fn propel(&mut self) {

		let distance = common::WORM_SPEED;
		
		let direction = self.heading * common::TAU;

		let x = direction.cos() * distance;
		let y = direction.sin() * distance;

		self.loc.x += x;
		self.loc.y += y;

	}
}
impl Worldly for EntityBody {
	fn scent(&self) -> Scent {
		match self.kind {
			EntityKind::Food =>	Scent { sweet: 100f32, sour: 0f32 },
			EntityKind::Poison => Scent::new(0f32, 100f32),
			EntityKind::Creature =>	Scent::new(0f32, 0f32),
			EntityKind::None => Scent::none(),
		}
	}

	fn loc(&self) -> Location {
		self.loc.clone()
	}

	fn name(&self) -> &str {
		self.name.as_slice()
	}

	fn kind(&self) -> EntityKind {
		self.kind.clone()
	}

}


pub enum EntityKind {
	None,
	Food,
	Poison,
	Creature,
}
impl Clone for EntityKind {
	fn clone(&self) -> EntityKind {
		match *self {
			EntityKind::None 			=> EntityKind::None,
			EntityKind::Food 			=> EntityKind::Food,
			EntityKind::Poison 			=> EntityKind::Poison,
			EntityKind::Creature 		=> EntityKind::Creature,
		}
	}
}
impl Show for EntityKind {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "(EntityKind:{})", 
			match *self {
					EntityKind::None 			=> "None",
					EntityKind::Food 			=> "Food",
					EntityKind::Poison 			=> "Poison",
					EntityKind::Creature 		=> "Creature",
			}
		)
	}
	
}


pub struct EntityBrain {
	body_uid: uint,
	scent_prev: Scent,
	just_turned_about: bool,
}
impl EntityBrain {
	pub fn new(body_uid: uint, world: &World) -> EntityBrain {

		EntityBrain { 
			body_uid: body_uid,
			scent_prev: world.sniff_from(body_uid),
			just_turned_about: false,
		}
	}

	pub fn print(&self) {
		print!("[Heading:, Previous Scent:{}] ", self.scent_prev);
	}
}
impl WormBrain for EntityBrain {
	fn act(&mut self, world: &mut World) -> Option<()> {

		let scent_new = world.sniff_from(self.body_uid);

		if scent_new.sweet == 0f32 {
			println!("Nothing else to eat");
			return Option::None;
		}

		self.navigate(world, scent_new.clone());
		self.propel(world);
		self.eat(world, &scent_new);

		Option::Some(())
	}

	fn navigate(&mut self, world: &mut World, scent_new: Scent) {
		let body = world.entities().get_mut(self.body_uid);
		if self.scent_prev.sweet > scent_new.sweet {
			if !self.just_turned_about {
				body.turn(0.25f32);
				self.just_turned_about = true;
			} else {
				body.turn(0.5f32);
				self.just_turned_about = false;
			}
		}

		self.scent_prev = scent_new;
	}
	
	fn eat(&mut self, world: &mut World, scent_new: &Scent) {
		//let body = world.entities().get_mut(0);
		
		if scent_new.sweet >= 1f32 {
			world.feed_entity(self.body_uid);
		}
	}

	fn propel(&self, world: &mut World) {
		let body = world.entities().get_mut(self.body_uid);
		body.propel();
	}
}
