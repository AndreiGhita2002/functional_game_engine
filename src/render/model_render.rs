use wgpu::{RenderBundle, RenderPipeline, TextureView};

use crate::asset::AssetStore;
use crate::asset::model::Model;
use crate::game::component::Component;
use crate::render::{GPUState, Renderer};
use crate::util::res::Res;
use crate::impl_component;

#[derive(Clone)]
pub struct ModelComponent {
    pub model: Res<Model>,
}

#[allow(dead_code)]
pub struct ModelRenderer {
    asset_store: Res<AssetStore>,
    gpu_state: Res<GPUState>,
    bundles: Vec<RenderBundle>,
    pipeline: RenderPipeline,
}

impl ModelRenderer {
    pub fn new(gpu_state: Res<GPUState>, _asset_store: Res<AssetStore>) -> Self {
        let _gpu = gpu_state.read().unwrap();
        todo!()
    }
}

impl Renderer for ModelRenderer {
    fn pre_render(&mut self) {
        todo!()
    }

    fn render_pass(&self, _view: &TextureView) {
        todo!()
    }
}

impl_component!(ModelComponent);
