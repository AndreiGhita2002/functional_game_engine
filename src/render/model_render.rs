use wgpu::{RenderBundle, RenderPipeline, TextureView};

use crate::asset::AssetStore;
use crate::asset::model::Model;
use crate::game::entity::{Component, Entity};
use crate::game::GameState;
use crate::render::{GPUState, Renderer};
use crate::util::res::Res;

#[derive(Clone)]
pub struct ModelComponent {
    pub model: Res<Model>,
    instance_id: u32,
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
    fn pre_render(&mut self, _game_state: &GameState) {
        todo!()
    }

    fn render_pass(&self, _view: &TextureView) {
        todo!()
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