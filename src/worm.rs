//use common;
use common::{ Scent };
use entity::{ EntityBrain, Mobile };
use world::{ World };
//use std::num::FloatMath;
//use std::fmt::{ Show, Formatter, Error };

pub trait WormBrain {
	fn act(&mut self, world: &mut World) -> Option<()>;
	fn navigate(&mut self, world: &mut World, scent_new: Scent);
	fn eat(&mut self, world: &mut World, scent_new: &Scent);
}

impl WormBrain for EntityBrain {
	fn act(&mut self, world: &mut World) -> Option<()> {

		let scent_new = world.sniff_from(self.body_uid);

		if scent_new.sweet == 0f32 {
			//println!("")
			//println!("Nothing else to eat");
			return Option::None;
		}

		self.navigate(world, scent_new.clone());
		world.entities().get_mut(self.body_uid).propel();
		self.eat(world, &scent_new);

		Option::Some(())
	}

	fn navigate(&mut self, world: &mut World, scent_new: Scent) {
		let body = world.entities().get_mut(self.body_uid);
		if self.scent_prev.sweet > scent_new.sweet {
			if !self.just_turned_about {
				body.turn(0.25f32, true);
				self.just_turned_about = true;
			} else {
				body.turn(0.5f32, true);
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

}
