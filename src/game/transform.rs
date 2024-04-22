use std::mem;
use wgpu::BufferAddress;
use crate::game::entity::{Component, Entity};
use crate::util::arena::Arena;
use crate::util::Either;

#[derive(Debug)]
pub struct Transform2D {
    pub pos: [f32; 2]
}

#[derive(Debug)]
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