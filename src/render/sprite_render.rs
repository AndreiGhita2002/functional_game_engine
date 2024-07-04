use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use wgpu::{RenderBundle, RenderBundleDescriptor, RenderPipeline, TextureView};

use crate::asset::{AssetStore};
use crate::asset::model::Material;
use crate::game::entity::{Component, Entity};
use crate::game::GameState;
use crate::game::transform::RawTransform2D;
use crate::render::{GPUState, Renderer, SpriteVertex, Vertex};
use crate::util::res::Res;

#[derive(Clone)]
pub struct SpriteComponent {
    material: Res<Material>,
    instance_id: u32,
}

pub struct SpriteRenderer {
    asset_store: Res<AssetStore>,
    gpu_state: Res<GPUState>,
    bundles: Vec<RenderBundle>,
    pipeline: RenderPipeline,
}

impl SpriteRenderer {
    pub fn new(gpu_state: Res<GPUState>, asset_store: Res<AssetStore>) -> Self {
        let pipeline = {
            let gpu = gpu_state.read().unwrap();
            let shader = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
            });

            let pipeline_layout = gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&gpu.bind_groups.texture_layout],
                push_constant_ranges: &[],
            });

            gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[SpriteVertex::desc(), RawTransform2D::desc::<2>()],
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
            })
        };
        SpriteRenderer {
            asset_store,
            gpu_state,
            bundles: Vec::new(),
            pipeline
        }
    }
}

impl Renderer for SpriteRenderer {
    /// Render Setup
    fn pre_render(&mut self, game: &GameState) {
        let gpu = self.gpu_state.read().unwrap();
        // borrow asset store
        let assets = self.asset_store.read().unwrap();
        // creating bundles
        let mut bundles = Vec::new();
        for entity in game.entities.iter() {
            if let Some(sprite) = entity.data().get::<SpriteComponent>("sprite") {
                let material = sprite.material.read().unwrap();
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
                encoder.set_bind_group(0, &material.bind_group, &[]);
                // pass a quad model in (two triangles make a square)
                encoder.set_vertex_buffer(0, assets.quad_v_buffer_slice(..));
                // pass the instance in
                encoder.set_vertex_buffer(1, assets.instance_buffer_2d_slice(..));
                // draw
                let i = sprite.instance_id..(sprite.instance_id + 1);
                encoder.draw(0..6, i);
                // output the bundle
                let bundle = encoder.finish(&RenderBundleDescriptor {
                    label: Some("sprite bundle"),
                });
                bundles.push(bundle);
            }
        }
        self.bundles = bundles
    }

    /// Render Pass
    fn render_pass(&self, view: &TextureView) {
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

impl Component for SpriteComponent {
    fn to_entity(mut self, entity: &mut Entity) {
        //todo THIS IS VERY BAD!!
        // make some kinda entity id to instance id mapping in AssetStore
        self.instance_id = entity.id() as u32;
        entity.mut_data().alloc(self, "sprite");
        eprintln!("{}" , entity.data().get_content_string());
    }
}

impl SpriteComponent {
    pub fn new(material: Res<Material>) -> Self {
        SpriteComponent { material, instance_id: 0 } // todo deal with instance id
    }
}

impl Display for SpriteComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mat_name = {
            let mat = self.material.read().unwrap();
            mat.name.clone()
        };
        write!(f, "Sprite[mat={},inst={}]", mat_name, self.instance_id)
    }
}