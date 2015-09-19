
//use num::Float;
use std::fmt::{ Formatter, Display };
use std::fmt::Result;
use std::clone::Clone;
use std::iter;
use std;

pub const PI: f32 = std::f32::consts::PI;
pub const TAU: f32 = PI * 2f32;

pub const WORM_SPEED: f32 = 0.1f32;
pub const ENTITY_VISIBLE_WIDTH: f32 = 10f32;
//pub const VISION_RESOLUTION: u32 = 1024u32;


//pub const PEEK_INITIAL_CAPACITY: u32 = 25;
//pub const PEEK_MAX_CAPACITY: u32 = 1024;

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
	/*fn clone_from(&mut self, source: &Location) { 
		self.x = source.x;
		self.y = source.y;
	}*/
}
impl Display for Location {
	fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(Location:(x:{})(y:{}))", self.x, self.y)
    }
}
impl Copy for Location { }


#[derive(Clone, Copy)]
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

	pub fn clone(&self) -> Scent {
		Scent {sweet: self.sweet, sour: self.sour}
	} 
	
}
impl Display for Scent {
	fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(sweet:{}, sour:{})", self.sweet, self.sour)
    }
}


pub struct Peek {
	pub peek: Vec<(u16, u8)>,
	pub width: u32,
}
impl Peek {
	pub fn new(width: u32) -> Peek {
		Peek { 
			peek: Vec::new(),
			width: width,
		 }
	}
	pub fn render_ent(&mut self, bear: f32, vis_size: u32, distance: f32) {
		let center: u32 = self.width + ((bear * self.width as f32).round().abs()) as u32;
		/*let center: u32 = self.width + std::num::cast::<f32, u32>((bear * std::num::cast::<u32, f32>(self.width).unwrap()).round().abs()).unwrap();*/
		let ent_radius: u32 = vis_size >> 1;
		let ent_radius_rem: u32 = vis_size & 1;


		let inten: u8 = if distance > 1023f32 {
			0u8
		} else {
			((1023f32 - distance)/8f32) as u8
			/*std::num::cast((1023f32 - distance)/8f32).unwrap()  */
		};

		for i in (center - ent_radius)..(center + ent_radius + ent_radius_rem) {
			let pixel = normalize_pixel(i, self.width);
			self.peek.push((pixel as u16, inten));
			/*self.peek.push((std::num::cast(pixel).unwrap(), inten));*/
		}
	}

	pub fn unfold(&self) -> Vec<u8> {
		let mut vec: Vec<u8> = iter::repeat(0).take(self.width as usize).collect();
		self.unfold_into(&mut vec, 0);
		vec
	}

	pub fn unfold_into(&self, vec: &mut Vec<u8>, offset: usize) {
		assert!(vec.len() >= (self.width as usize + offset));

		for x in vec.iter_mut() {
		    *x = 0;
		}

		for &p in self.peek.iter() {
			let (idx, state) = p;
			vec[idx as usize + offset] = state;
		}
	}

	/*pub fn width(&self) -> u8 {
		self.peek.len()
	}*/
}

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
    dist
}

pub fn distance(loc_a: &Location, loc_b: &Location) -> f32 {
    let x_delta = loc_b.x - loc_a.x;
    let y_delta = loc_b.y - loc_a.y;
    (x_delta.powi(2i32) + y_delta.powi(2i32)).sqrt()
}

pub fn bearing(loc_a: &Location, loc_b: &Location) -> f32 {
	let x_delta = loc_b.x - loc_a.x;
    let y_delta = loc_b.y - loc_a.y;

    let bearing = 1f32 - ((y_delta.atan2(x_delta) / (TAU)) + 0.25f32);

    normalize_bearing(bearing)

}

pub fn ang_dia(dist: f32, dia: f32) -> f32 {
	(dia / (2f32 * dist)).atan() / PI  
}

pub fn vis_size(dist: f32, world_width: u32) -> u32 {
	(ang_dia(dist, ENTITY_VISIBLE_WIDTH) * (world_width as f32)).round().abs() as u32
}

pub fn normalize_bearing(mut bearing: f32) -> f32 {
	while bearing >= 1f32 {
		bearing -= 1f32;
	}
	while bearing < 0f32 {
		bearing += 1f32;
	}
	bearing
}

pub fn normalize_pixel(mut pixel: u32, world_width: u32) -> u32 {
	while pixel >= world_width {
		pixel -= world_width;
	}
		
	pixel
}




     /*
	((x_delta).sin() * (loc_b.y).cos()).atan2((loc_a.y).cos() * (loc_b.y).sin() - (loc_a.y).sin() * (loc_b.y).cos() * (x_delta).cos()) / (PI)
	*/
