use std::mem;
use crate::game::entity::{Component, Entity};
use crate::util::arena::Arena;

pub struct Transform2D {
    pub pos: [f32; 2]
}

pub struct Transform3D {
    pub pos: [f32; 3]
}

pub fn get_pos(arena: &Arena) -> Option<Either<Transform2D, Transform3D>> {
    let l = arena.get_length("pos")?;

    if l == mem::size_of::<Transform2D>() {
        let t: Transform2D = arena.get("pos")?;
        Some(Either::This(t))

    } else if l == mem::size_of::<Transform3D>() {
        let t: Transform3D = arena.get("pos")?;
        Some(Either::That(t))

    } else {
        // unknown size of transform found
        None
    }
}

impl Component for Transform2D {
    fn to_entity(self, entity: &mut Entity) {
        entity.mut_data().alloc(self, "pos")
    }
}

impl Component for Transform3D {
    fn to_entity(self, entity: &mut Entity) {
        entity.mut_data().alloc(self, "pos")
    }
}