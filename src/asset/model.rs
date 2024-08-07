use std::fmt::Debug;

use wgpu::Device;
use wgpu::util::DeviceExt;

use crate::{GPUState};
use crate::asset::{MaterialId, resources};
use crate::render::{ModelVertex, Vertex};
use crate::asset::texture::Texture;

pub struct Material {
    pub name: String,
    pub diffuse_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: MaterialId,
}

impl Mesh {
    pub fn from_vertices<T: Vertex>(
        vertices: Vec<T>,
        indices: Vec<u32>,
        name: &str,
        material: usize,
        device: &Device,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Vertex Buffer", name)),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Index Buffer", name)),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        // println!("  vertex: {:?}\n  index: {:?}", vertex_buffer, index_buffer);
        Mesh {
            name: name.to_string(),
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material,
        }
    }
}

impl Material {
    pub fn from_texture(mat_name: &str, texture: Texture, context: &GPUState) -> Material {
        let bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &context.bind_groups.texture_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    },
                ],
                label: None,
            });
        Material {
            name: mat_name.to_string(),
            diffuse_texture: texture,
            bind_group,
        }
    }

    pub fn from_texture_file(filename: &str, context: &GPUState) -> Material {
        let f = async { resources::load_texture(filename, &context.device, &context.queue).await };
        let diffuse_texture = pollster::block_on(f).unwrap();
        Self::from_texture(filename, diffuse_texture, context)
    }
}

impl Model {
    pub fn from_model_file(filename: &str, context: &GPUState) -> anyhow::Result<Model> {
        let f = async { resources::load_model(
            filename,
            &context.device,
            &context.queue,
            &context.bind_groups.texture_layout
        ).await };
        pollster::block_on(f)
    }
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct ModelBlueprint {
    pub name: String,
    pub diffuse_texture_name: String,
    pub vertices: Vec<(f32, f32, f32, f32, f32)>,
    pub indices: Vec<u32>,
}
#[allow(dead_code)]
impl ModelBlueprint {
    pub fn into_model(self, context: &GPUState) -> (String, Model) {
        let mesh_vertices = self
            .vertices
            .iter()
            .map(|vertex| ModelVertex {
                position: [vertex.0, vertex.1, vertex.2],
                tex_coords: [vertex.0, vertex.1],
                normal: [0.0, 0.0, 0.0],
            })
            .collect::<Vec<_>>();

        let mesh = Mesh::from_vertices(
            mesh_vertices,
            self.indices,
            &self.name,
            0,
            &context.device,
        );

        let material = Material::from_texture_file(&self.diffuse_texture_name, context);

        let model = Model {
            meshes: vec![mesh],
            materials: vec![material],
        };

        (self.name, model)
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut meshes = format!("({}", self.meshes[0]);
        for mesh in self.meshes.iter().skip(1) {
            let new_meshes = format!("{}, {}", meshes, mesh);
            meshes = new_meshes;
        }
        meshes = format!("{})", meshes);
        let mut materials = format!("({}", self.materials[0]);
        for material in self.materials.iter().skip(1) {
            materials = format!("{}, {}", materials, material);
        }
        write!(f, "model[meshes: {}, materials: {}]", meshes, materials)
    }
}

impl std::fmt::Display for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mesh:{}", self.name)
    }
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mesh:{}", self.name)
    }
}
