use std::f32::consts::PI;
use std::fmt::{Display, Formatter};
use std::mem;

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Quaternion, Vector3};
use cgmath::num_traits::Pow;
use mem_macros::size_of;
use wgpu::BufferAddress;

use crate::game::component::{Component, impl_component};
use crate::util::arena::ComponentArena;
use crate::util::Either;

pub const TRANSFORM_COMP_NAME: &str = "pos";

#[derive(Copy, Clone, Debug)]
pub struct Transform2D {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub rot: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Transform3D {
    pub pos: [f32; 3],
    pub size: [f32; 3],
    pub rotation: Quaternion<f32>,
}

impl Transform2D {
    pub fn dist(t1: Transform2D, t2: Transform2D) -> f32 {
        return f32::sqrt((t2.pos[0] - t1.pos[0]).pow(2) + (t2.pos[1] - t1.pos[1]).pow(2))
    }

    pub fn to_raw(&self) -> RawTransform2D {
        let cos_r = (self.rot * PI).cos();
        let sin_r = (self.rot * PI).sin();
        return RawTransform2D {
            offset: self.pos,
            matrix: [
                [self.size[0] * cos_r, -sin_r],
                [sin_r, self.size[1] * cos_r],
            ]
        }
    }
}

impl Transform3D {
    pub fn dist(t1: Transform3D, t2: Transform3D) -> f32 {
        return f32::sqrt((t2.pos[0] - t1.pos[0]).pow(2)
            + (t2.pos[1] - t1.pos[1]).pow(2) + (t2.pos[2] - t1.pos[2]).pow(2))
    }

    pub fn to_raw(&self) -> RawTransform3D {
        return RawTransform3D {
            model: (Matrix4::from_translation(Vector3::from(self.pos))
                * Matrix4::from(self.rotation)).into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }
}

pub fn get_pos(arena: &ComponentArena) -> Option<Either<Transform2D, Transform3D>> {
    let l = arena.get_length(TRANSFORM_COMP_NAME)?;

    if l == mem::size_of::<Transform2D>() {
        let t: Transform2D = arena.get(TRANSFORM_COMP_NAME)?;
        Some(Either::This(t))

    } else if l == mem::size_of::<Transform3D>() {
        let t: Transform3D = arena.get(TRANSFORM_COMP_NAME)?;
        Some(Either::That(t))

    } else {
        // unknown size of transform found
        None
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct RawTransform2D {
    pub offset: [f32; 2],
    pub matrix: [[f32; 2]; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct RawTransform3D {
    pub model: [[f32; 4]; 4],
    pub normal: [[f32; 3]; 3],
}

impl RawTransform2D {
    pub fn desc<'a, const LOC: u32>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RawTransform2D>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {  // position
                    offset: 0,
                    shader_location: LOC,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {  // matrix row 0
                    offset: size_of!([f32; 2]) as BufferAddress,
                    shader_location: LOC + 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {  // matrix row 1
                    offset: size_of!([f32; 4]) as BufferAddress,
                    shader_location: LOC + 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

impl RawTransform3D {
    pub fn desc<'a, const LOC: u32>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RawTransform3D>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: LOC,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in
                // the shader.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: LOC + 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: LOC + 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: LOC + 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as BufferAddress,
                    shader_location: LOC + 4,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 19]>() as BufferAddress,
                    shader_location: LOC + 5,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 22]>() as BufferAddress,
                    shader_location: LOC + 6,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

impl_component!(Transform2D);
impl_component!(Transform3D);

impl Display for Transform2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.pos[0], self.pos[1])
    }
}