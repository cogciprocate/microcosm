use std::f32;
use std::num::Float;

pub const PI: f32 = f32::consts::PI;

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
		/*
		Scent { 
			sweet: scent.sweet + self.sweet, 
			sour: scent.sour + self.sour,
		}
		*/
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

pub fn floats_eq(a: f32, b: f32) -> bool {
	let ep = 0.0001f32;
	if (a - b).abs() < ep {
		true
	} else {
		false
	}
}

/*
pub fn print_sniffs(sniffs: Vec<Sniff>) {

	for sniff in sniffs.iter() {
		//let (ref scent, intensity): (Scent, f32) = *sniff;
		//let (scent, intensity): (Scent, f32) = sniff.clone(); 
		println!("scent.sweet: {}, scent.sour: {}, intensity: {}, direction: {}", sniff.scent.sweet, sniff.scent.sour, sniff.intensity, sniff.direction);
		//println!("Sniff happens");
	}
}
*/
