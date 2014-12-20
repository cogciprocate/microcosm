use entity::{ Entity, EntityKind };
use common::{ Location };

pub struct Creature<'a> {
	entity: Entity,
}

impl <'a>Creature<'a> {
	pub fn new(name: String, kind: EntityKind, loc: Location) -> Creature<'a> {
		Creature { entity: Entity::new(name, kind, loc) }
	}
}

/*
impl <'a> Entity for Creature<'a> {

}
*/

