use std::borrow::Cow;
use bytemuck::{Pod, Zeroable};
use wgpu::{Buffer, BufferAddress, BufferDescriptor, BufferUsages, RenderBundle, RenderBundleDescriptor, RenderPipeline, TextureView};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::asset::AssetStore;
use crate::asset::model::Model;
use crate::game::entity::{Component, Entity};
use crate::game::GameState;
use crate::game::transform::{RawTransform3D};
use crate::render::{BindGroups, GPUState, ModelVertex, Renderer, Vertex};
use crate::util::res::Res;

#[derive(Clone)]
pub struct ModelComponent {
    pub model: Res<Model>,
    instance_id: u32,
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, Zeroable, Pod)]
pub struct CameraBufferData {
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
}

pub struct ModelRenderer {
    asset_store: Res<AssetStore>,
    gpu_state: Res<GPUState>,
    bundles: Vec<RenderBundle>,
    pipeline: RenderPipeline,
    camera_buffer_data: CameraBufferData,
    camera_buffer: Buffer,
    camera_bind_group: wgpu::BindGroup,
}

impl ModelRenderer {
    pub fn new(gpu_state: Res<GPUState>, asset_store: Res<AssetStore>) -> Self {
        let (camera_buffer_data, camera_buffer, pipeline, camera_bind_group) = {
            let gpu = gpu_state.read().unwrap();
            let vp_buffer_data = CameraBufferData::default();

            let vp_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
                label: Some("MVP Buffer"),
                contents: bytemuck::bytes_of(&vp_buffer_data),
                usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            });

            let camera_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &gpu.bind_groups.camera_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(vp_buffer.as_entire_buffer_binding()),
                    },
                ],
                label: None,
            });

            let shader = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("model.wgsl"))),
            });

            let pipeline_layout = gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&gpu.bind_groups.texture_layout, &gpu.bind_groups.camera_layout],
                push_constant_ranges: &[],
            });

            let pipeline = gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Model Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[ModelVertex::desc(), RawTransform3D::desc::<2>()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(gpu.surface_format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });
            (vp_buffer_data, vp_buffer, pipeline, camera_bind_group)
        };
        ModelRenderer {
            asset_store,
            gpu_state,
            bundles: Vec::new(),
            pipeline,
            camera_buffer_data,
            camera_buffer,
            camera_bind_group,
        }
    }
}

impl Renderer for ModelRenderer {
    fn pre_render(&mut self, game: &GameState) {
        let gpu = self.gpu_state.read().unwrap();
        // borrow asset store
        let assets = self.asset_store.read().unwrap();
        // creating bundles
        let mut bundles = Vec::new();
        for entity in game.entities.iter() {
            if let Some(model_comp) = entity.data().get::<ModelComponent>("model") {
                let i = model_comp.instance_id;
                if let Ok(model) = model_comp.model.read() {
                    for mesh in model.meshes.iter() {
                        let material = model.materials.get(mesh.material).unwrap();
                        // create the encoder
                        let mut encoder = gpu.device.create_render_bundle_encoder(
                            &wgpu::RenderBundleEncoderDescriptor {
                                label: Some("Bundle Encoder"),
                                color_formats: &[Some(gpu.surface_format)],
                                depth_stencil: None,
                                sample_count: 1,  //wgpu::MultisampleState::default() has count 1
                                multiview: None,
                            }
                        );
                        // setting the pipeline
                        encoder.set_pipeline(&self.pipeline);
                        // pass the texture in
                        encoder.set_bind_group(1, &material.bind_group, &[]);
                        // pass the vertex buffer
                        encoder.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                        // pass the index buffer
                        encoder.set_index_buffer(mesh.index_buffer.slice(..),
                                                 wgpu::IndexFormat::Uint32);
                        // pass the model matrix
                        encoder.set_vertex_buffer(2, assets.instance_buffer_3d_slice(..));
                        // pass the camera uniform
                        encoder.set_bind_group(0, &self.camera_bind_group, &[]);
                        // draw
                        encoder.draw(0..mesh.num_elements, i..(i+1));
                        // output
                        let bundle = encoder.finish(&RenderBundleDescriptor {
                            label: Some("model bundle"),
                        });
                        bundles.push(bundle);
                    }
                }
            }
        }
        self.bundles = bundles
    }

    fn render_pass(&self, view: &TextureView) {
        //todo set the camera uniform
        let gpu = self.gpu_state.read().unwrap(); //todo deadlock?
        let mut command_encoder = gpu.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("Command Encoder") }
        );
        // render pass
        {
            let mut render_pass = command_encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.execute_bundles(self.bundles.iter());
        }
        gpu.queue.submit(Some(command_encoder.finish()));
    }
}

impl Component for ModelComponent {
    fn to_entity(mut self, entity: &mut Entity) {
        //todo THIS IS also VERY BAD!!
        self.instance_id = entity.id() as u32;
        entity.mut_data().alloc(self, "model");
        eprintln!("{}" , entity.data().get_content_string());
    }
}

impl ModelComponent {
    pub fn new(model: Res<Model>) -> Self {
        ModelComponent { model, instance_id: 0 }
    }
}
