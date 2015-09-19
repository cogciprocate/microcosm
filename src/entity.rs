
use common;
use common::{ Location, Scent };
use world::{ World };
use num::Float;
use std::fmt::{ Formatter, Error, Display };
//use std::num::Float;
//use std::collections::HashMap;

pub trait Worldly {
	fn scent(&self) -> Scent;
	fn loc(&self) -> Location;
	fn name(&self) -> &'static str;
	fn kind(&self) -> EntityKind;
}

pub trait Mobile {
	fn propel(&mut self);
	fn turn(&mut self, amt: f32, turn_left: bool);
	fn heading(&self) -> f32;
	fn head_north(&mut self);
}


#[derive(Clone, Copy)]
pub struct EntityBody {
	pub name: &'static str,
	loc: Location,
	pub heading: f32,
	kind: EntityKind,
	pub eaten: bool,
	pub uid: usize,
}

impl EntityBody {
	pub fn new(name: &'static str, kind: EntityKind, loc: Location) -> EntityBody {
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

impl Display for EntityBody {
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
	fn turn(&mut self, amt: f32, turn_left: bool) {
		if turn_left {
			self.heading += amt;
		} else {
			self.heading -= amt;
		}
		self.heading = common::normalize_bearing(self.heading);
	}

	fn head_north(&mut self) {
		self.heading = 0f32;
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

	fn name(&self) -> &'static str {
		&self.name
	}

	fn kind(&self) -> EntityKind {
		self.kind.clone()
	}

}

#[derive(Clone, Copy)]
pub enum EntityKind {
	None,
	Food,
	Poison,
	Creature,
}

impl Display for EntityKind {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "EntityKind:{}", 
			match *self {
					EntityKind::None 			=> "None",
					EntityKind::Food 			=> "Food",
					EntityKind::Poison 			=> "Poison",
					EntityKind::Creature 		=> "Creature",
			}
		)
	}
	
}


#[derive(Clone, Copy)]
pub struct EntityBrain {
	pub body_uid: usize,
	pub scent_prev: Scent,
	pub just_turned_about: bool,
}
impl EntityBrain {
	pub fn new(body_uid: usize, world: &World) -> EntityBrain {

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
