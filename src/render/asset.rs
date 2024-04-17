use std::ops::RangeBounds;
use std::sync::{Arc, Mutex};
use wgpu::{Buffer, BufferAddress, BufferDescriptor, BufferSlice, Device};
use wgpu::util::DeviceExt;
use crate::game::GameState;
use crate::game::transform::{Either, get_pos};
use crate::render::model::Material;
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
    pub fn new(device: &Device) -> Res<Self> {
        Res::new(AssetStore {
            materials: Vec::new(),
            instance_buffer_2d: device.create_buffer(&BufferDescriptor {
                label: Some("2D Instance Buffer"),
                size: 0,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            instance_buffer_3d: device.create_buffer(&BufferDescriptor {
                label: Some("3D Instance Buffer"),
                size: 0,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
        })
    }
}

impl Res<AssetStore> {
    pub fn get_material(&self, id: MaterialId) -> Option<&Material> {
        self.read().unwrap().materials.get(id)
    }

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
            let store = self.write().unwrap();
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

    pub fn instance_buffer_2d_slice<S: RangeBounds<BufferAddress>>(&self, range: S) -> BufferSlice<'_> {
        self.write().unwrap().instance_buffer_2d.slice(range)
    }
}
