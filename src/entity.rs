

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
	fn move_towards(&mut self, direction: f32, magnitude: f32);
}


pub struct EntityBody {
	name: String,
	loc: Location,
	kind: EntityKind,
	pub eaten: bool,
	pub uid: uint,
}
impl EntityBody {
	pub fn new(name: String, kind: EntityKind, loc: Location) -> EntityBody {
		EntityBody {
			name: name,
			loc: loc,
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
	fn move_towards(&mut self, dir: f32, distance: f32) {
		// direction is in radians where up (north) is pi/2 and right (east) is 0
		let direction = dir * common::TAU;

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
	scent_prev: Scent,
	just_turned_about: bool,
	heading: f32,
}
impl EntityBrain {
	pub fn new(scent_init: Scent) -> EntityBrain {
		EntityBrain { 
			scent_prev: scent_init,
			just_turned_about: false,
			heading: 0.30f32,
		}
	}

	pub fn act(&mut self, world: &mut World) -> Option<()> {

		let scent_new = world.sniff_at_entity(0);

		if scent_new.sweet == 0f32 {
			println!("Nothing else to eat");
			return Option::None;
		}

		self.navigate(scent_new);		
		self.move_body(world);
		self.eat(world);

		Option::Some(())
	}

	fn navigate(&mut self, scent_new: Scent) {
		if self.scent_prev.sweet > scent_new.sweet {
			if !self.just_turned_about {
				self.heading += 0.25f32;
				self.just_turned_about = true;
			} else {
				self.heading += 0.5f32;
				self.just_turned_about = false;
			}
		}

		self.scent_prev = scent_new;
		if self.heading > 1f32 { self.heading -= 1f32 };
	}

	fn move_body(&self, world: &mut World) {
		let body = world.entities().get_mut(0);
		body.move_towards(self.heading, common::WORM_SPEED);
	}

	fn eat(&mut self, world: &mut World) {
		let scent_new: Scent = world.sniff_at_entity(0);
		//let body = world.entities().get_mut(0);
		
		if scent_new.sweet >= 1f32 {
			world.feed_entity(0);
		}
	}
	
	pub fn print(&self) {
		print!("[Heading:{}, Previous Scent:{}] ", self.heading, self.scent_prev);
	}
	
}
