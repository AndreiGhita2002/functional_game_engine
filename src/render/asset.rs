use std::ops::RangeBounds;
use wgpu::{Buffer, BufferAddress, BufferDescriptor, BufferSlice, Device};
use wgpu::util::DeviceExt;
use crate::game::GameState;
use crate::game::transform::{get_pos};
use crate::render::GPUState;
use crate::render::model::Material;
use crate::util::Either;
use crate::util::res::Res;

//todo move asset.rs out of render module

pub type MaterialId = usize;

pub struct AssetStore {
    // materials
    materials: Vec<Material>,
    // instances
    pub instance_buffer_2d: Buffer,
    pub instance_buffer_3d: Buffer,
}

impl AssetStore {
    pub fn new(gpu: &GPUState, to_load: Option<AssetsToLoad>) -> Res<Self> {
        let mut materials = Vec::new();
        if let Some(file_queue) = to_load {
            for filename in file_queue.texture_files.iter() {
                let mat = Material::from_texture_file(filename, gpu);
                materials.push(mat);
            }
        }
        Res::new(AssetStore {
            materials,
            instance_buffer_2d: gpu.device.create_buffer(&BufferDescriptor {
                label: Some("2D Instance Buffer"),
                size: 0,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            instance_buffer_3d: gpu.device.create_buffer(&BufferDescriptor {
                label: Some("3D Instance Buffer"),
                size: 0,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
        })
    }

    pub fn get_material(&self, id: MaterialId) -> Option<&Material> {
        self.materials.get(id)
    }

    pub fn instance_buffer_2d_slice<S: RangeBounds<BufferAddress>>(&self, range: S) -> BufferSlice<'_> {
        self.instance_buffer_2d.slice(range).clone()
    }
}

impl Res<AssetStore> {
    pub fn update_from_game(&mut self, game_state: &GameState, device: &Device) {
        let mut raw2d = Vec::new();
        let mut raw3d = Vec::new();

        for entity in game_state.entities.iter() {
            if let Some(pos) = get_pos(entity.data()) {
                match pos {
                    Either::This(t_2d) => {raw2d.push(t_2d.pos)}
                    Either::That(t_3d) => {raw3d.push(t_3d.pos)}
                }
            }
        }

        {
            let mut store = self.write().unwrap();
            store.instance_buffer_2d = device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("2D Instance Buffer"),
                    contents: bytemuck::cast_slice(&raw2d),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
            store.instance_buffer_3d = device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("3D Instance Buffer"),
                    contents: bytemuck::cast_slice(&raw3d),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
        }
    }
}


pub struct AssetsToLoad {
    pub texture_files: Vec<String>
}
