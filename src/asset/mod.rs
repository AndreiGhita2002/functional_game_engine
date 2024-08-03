use std::mem;
use std::ops::{Deref, RangeBounds};

use wgpu::{Buffer, BufferAddress, BufferDescriptor, BufferSlice};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use model::{Material, Model};

use crate::game::{GAME_STATE, GameState};
use crate::game::transform::{get_pos, RawTransform2D, RawTransform3D};
use crate::render::{GPUState, SpriteVertex};
use crate::util::Either;
use crate::util::res::Res;

pub mod model;
pub mod resources;
pub mod texture;

// QUAD constant
const SQUARE_MESH: [SpriteVertex; 6] = [
    SpriteVertex { position: [0., 0.], tex_coords: [1., 1.] },
    SpriteVertex { position: [1., 0.], tex_coords: [0., 1.] },
    SpriteVertex { position: [1., 1.], tex_coords: [0., 0.] },
    SpriteVertex { position: [0., 0.], tex_coords: [1., 1.] },
    SpriteVertex { position: [1., 1.], tex_coords: [0., 0.] },
    SpriteVertex { position: [0., 1.], tex_coords: [1., 0.] },
];

pub type MaterialId = usize; //todo not that necessary now, you should remove it

/// Main implementation of Asset Manager
pub struct AssetStore {
    materials: Vec<Res<Material>>,
    models: Vec<Res<Model>>,
    gpu_state: Res<GPUState>,
    // instances
    pub instance_buffer_2d: Buffer,
    pub instance_buffer_3d: Buffer,
    // quad buffer
    pub quad_vertex_buffer: Buffer,
}

impl AssetStore {
    pub fn new(gpu_state: Res<GPUState>) -> Res<Self> {
        let gpu_state_clone = gpu_state.clone();
        let gpu = gpu_state.read().unwrap();
        Res::new(AssetStore {
            materials: vec![],
            models: vec![],
            gpu_state: gpu_state_clone,
            instance_buffer_2d: gpu.device.create_buffer(&BufferDescriptor {
                label: Some("2D Instance Buffer"),
                size: mem::size_of::<RawTransform2D>() as BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            instance_buffer_3d: gpu.device.create_buffer(&BufferDescriptor {
                label: Some("3D Instance Buffer"),
                size: mem::size_of::<RawTransform3D>() as BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            quad_vertex_buffer: gpu.device.create_buffer_init(&BufferInitDescriptor {
                label: Some("Quad Vertex Buffer"),
                contents: bytemuck::cast_slice(&SQUARE_MESH),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            })
        })
    }

    pub fn load_material(&mut self, filename: &str) -> Res<Material> {
        let g = self.gpu_state.read().unwrap();
        let material = Material::from_texture_file(filename, g.deref());

        let r_material = Res::new(material);
        self.materials.push(r_material.clone());
        r_material
    }

    pub fn load_model(&mut self, filename: &str) -> Res<Model> {
        let g = self.gpu_state.read().unwrap();

        let mut error_str = String::from("Model file not found: ");
        error_str.push_str(filename);
        let model = Model::from_model_file(filename, g.deref())
            .expect(&error_str);

        let r_model = Res::new(model);
        self.models.push(r_model.clone());
        r_model
    }

    pub fn get_material(&self, id: MaterialId) -> Option<&Res<Material>> {
        self.materials.get(id)
    }

    pub fn get_material_by_name(&self, material_name: &str) -> Option<&Res<Material>> {
        self.materials.iter()
            .find(|mat| mat.read().unwrap().name == material_name)
    }

    pub fn get_material_id(&self, material_name: &str) -> Option<MaterialId> {
        for i in 0..self.materials.len() {
            let mat = self.materials.get(i).unwrap().read().unwrap();
            if mat.name == material_name {
                return Some(i as MaterialId);
            }
        }
        None
    }

    pub fn instance_buffer_2d_slice<S: RangeBounds<BufferAddress>>(&self, range: S) -> BufferSlice<'_> {
        self.instance_buffer_2d.slice(range)
    }

    pub fn quad_v_buffer_slice<S: RangeBounds<BufferAddress>>(&self, range: S) -> BufferSlice<'_> {
        self.quad_vertex_buffer.slice(range)
    }
}

impl Res<AssetStore> {
    pub fn update_from_game(&mut self, gpu: &GPUState) {
        let game_state = GAME_STATE.lock().unwrap();
        let mut raw2d = Vec::new();
        let mut raw3d = Vec::new();

        for entity in game_state.entities.iter() {
            if let Some(pos) = get_pos(entity.data()) {
                match pos {
                    Either::This(t_2d) => {raw2d.push(t_2d.to_raw())}
                    Either::That(t_3d) => {raw3d.push(t_3d.to_raw())}
                }
            }
        }
        {
            let mut store = self.write().unwrap();
            // let gpu = store.gpu_state.read().unwrap();
            store.instance_buffer_2d = gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: Some("2D Instance Buffer"),
                    contents: bytemuck::cast_slice(&raw2d),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
            store.instance_buffer_3d = gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: Some("3D Instance Buffer"),
                    contents: bytemuck::cast_slice(&raw3d),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
        }
    }
}
