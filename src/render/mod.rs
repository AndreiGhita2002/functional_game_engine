use std::borrow::Cow;
use std::rc::Rc;
use wgpu::{RenderBundle, RenderBundleEncoder, RenderPass, RenderPipeline, SurfaceTargetUnsafe, TextureFormat};
use winit::window::Window;
use crate::game::entity::Entity;
use crate::game::GameState;
use crate::render::asset::AssetStore;
use crate::util::res::Res;

pub mod render_fn;
pub mod texture;
pub mod model;
pub mod asset;

// todo RenderFn: more intelligent AssetStore passing
// todo RenderFn: return an enum that states how many times this function should be called
//  remember that render bundles could be used for multiple renderPasses as a optimisation
// todo RenderFn: return something that indicates what render pipeline to use
pub type RenderFn = fn(&Entity, Res<AssetStore>, RenderBundleEncoder) -> RenderBundle;

pub struct BindGroups {
    pub texture_layout: wgpu::BindGroupLayout,
    // pub camera_layout: wgpu::BindGroupLayout,
    // pub light_layout: wgpu::BindGroupLayout,
    // pub camera: wgpu::BindGroup,
    // pub light: wgpu::BindGroup,
}

pub struct GPUState<'w> {
    surface: wgpu::Surface<'w>,
    pub surface_format: TextureFormat,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipeline: RenderPipeline,
    bind_groups: BindGroups,
}

impl GPUState<'_> {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // let surface = instance.create_surface(&window).unwrap();
        let surface = unsafe {
            let surface_target = SurfaceTargetUnsafe::from_window(&window).unwrap();
            instance.create_surface_unsafe(surface_target).unwrap()
        };

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web, we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, //todo try out the trace path
        ).await.unwrap();

        // bind groups:
        let texture_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&texture_layout],
            push_constant_ranges: &[],
        });

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(surface_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
        surface.configure(&device, &config);

        GPUState {
            surface, surface_format,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            bind_groups: BindGroups {texture_layout},
        }
    }

    pub fn render(&mut self, game_state: &GameState, asset_store: Res<AssetStore>) {
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        // creating bundles
        let mut bundles = Vec::new();
        for entity in game_state.entities.iter() {
            let bundle_encoder = self.device.create_render_bundle_encoder(
                &wgpu::RenderBundleEncoderDescriptor {
                    label: Some("Bundle Encoder"),
                    color_formats: &[Some(self.surface_format)],
                    depth_stencil: None,
                    sample_count: 0,
                    multiview: None,
                }
            );
            let maybe_bundle = entity.render(asset_store.clone(), bundle_encoder);
            if let Some(bundle) = maybe_bundle {
                bundles.push(bundle);
            }
        }
        // command encoder
        let mut command_encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("Command Encoder") }
        );
        // render pass
        {
            let mut render_pass = command_encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.execute_bundles(bundles.iter());
        }
        self.queue.submit(Some(command_encoder.finish()));
        frame.present();
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
