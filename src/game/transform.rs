use std::fmt::{Display, Formatter};
use std::mem;
use cgmath::num_traits::Pow;

use wgpu::BufferAddress;

use crate::game::entity::{Component, Entity};
use crate::util::arena::ComponentArena;
use crate::util::Either;

#[derive(Copy, Clone, Debug)]
pub struct Transform2D {  //todo: Describe size and rotation!
    pub pos: [f32; 2]
}

#[derive(Copy, Clone, Debug)]
pub struct Transform3D {
    pub pos: [f32; 3]
}

impl Transform2D {
    pub fn desc<'a, const LOC: u32>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Transform2D>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: LOC,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }

    pub fn dist(t1: Transform2D, t2: Transform2D) -> f32 {
        return f32::sqrt((t2.pos[0] - t1.pos[0]).pow(2) + (t2.pos[1] - t1.pos[1]).pow(2))
    }
}

pub fn get_pos(arena: &ComponentArena) -> Option<Either<Transform2D, Transform3D>> {
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

impl Display for Transform2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.pos[0], self.pos[1])
    }
}