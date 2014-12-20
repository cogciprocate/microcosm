
use std::f32;
use std::num::Float;
use std::fmt::{ Show, Formatter };
use std::fmt::Result;
use std::clone::Clone;

//pub const PI: f32 = f32::consts::PI;
pub const TAU: f32 = f32::consts::PI_2;

pub const WORM_SPEED: f32 = 0.1f32;

/*
pub struct Sniff {
	pub scent: Scent,
	pub intensity: f32,
	pub direction: f32,
}
impl Sniff {
	pub fn new(scent: Scent, intensity: f32, direction:f32) -> Sniff {
		Sniff {
			scent: scent,
			intensity: intensity,
			direction: direction,
		}
	}

}
*/


pub struct Location {
	pub x: f32,
	pub y: f32,
}
impl Location {
	pub fn origin() -> Location {
		Location { x: 0f32, y: 0f32 }
	}

	pub fn new(x: f32, y: f32) -> Location {
		Location { x: x, y: y }
	}
}
impl Clone for Location {
	fn clone(&self) -> Location {
		Location { x: self.x, y: self.y }
	}
	fn clone_from(&mut self, source: &Location) { 
		self.x = source.x;
		self.y = source.y;
	}
}
impl Show for Location {
	fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(Location:(x:{})(y:{}))", self.x, self.y)
    }
}

pub struct Scent {
	pub sweet: f32,
	pub sour: f32,
}
impl Scent {
	pub fn new(sweet: f32, sour: f32) -> Scent {
		Scent { sweet: sweet, sour: sour}
	}

	pub fn none() -> Scent {
		Scent { sweet: 0f32, sour: 0f32}
	}

	pub fn add(&mut self, scent: Scent) {
		self.sweet += scent.sweet;
		self.sour += scent.sour;
	}

	pub fn scale(&mut self, inten: f32) {
		self.sweet *= inten;
		self.sour *= inten;
	}

	/*
	fn clone(&self) -> Scent {
		Scent {sweet: self.sweet, sour: self.sour}
	} 
	*/
}
impl Show for Scent {
	fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(sweet:{}, sour:{})", self.sweet, self.sour)
    }
}


/*
pub struct Heading {
	pub dir: f32,
}
impl Heading {
	pub fn new(dir: f32) -> Heading {
		Heading { dir: f32 }
	}

	pub fn yaw(amt: f32) {

	}

	//pub fn pitch() {}
}
*/


pub fn floats_eq(a: f32, b: f32) -> bool {
	let ep = 0.0001f32;
	if (a - b).abs() < ep {
		true
	} else {
		false
	}
}

pub fn distance_between(loc_a: &Location, loc_b: &Location) -> f32 {
    let x_delta = loc_b.x - loc_a.x;
    let y_delta = loc_b.y - loc_a.y;
    let dist = (x_delta.powi(2i32) + y_delta.powi(2i32)).sqrt();

    //println!("*** x_delta: {}; x_delta.abs(): {} ***", x_delta, x_delta.abs());
    //println!("*** y_delta: {}; y_delta.abs(): {} ***", y_delta, y_delta.abs());
    //println!("*** distance: {} ***", dist);
    dist
}
