use wgpu::{RenderBundle, RenderPipeline, TextureView};

use crate::asset::AssetStore;
use crate::game::entity::{Component, Entity};
use crate::game::GameState;
use crate::render::{GPUState, Renderer};
use crate::render::model::Model;
use crate::util::res::Res;

#[derive(Clone)]
pub struct ModelComponent {
    pub model: Res<Model>,
    instance_id: u32,
}

pub struct ModelRenderer {
    asset_store: Res<AssetStore>,
    bundles: Vec<RenderBundle>,
    pipeline: RenderPipeline,
}

impl ModelRenderer {
    pub fn new(_gpu: &GPUState, _asset_store: Res<AssetStore>) -> Self {
        todo!()
    }
}

impl Renderer for ModelRenderer {
    fn pre_render(&mut self, _gpu_state: &GPUState, _game_state: &GameState) {
        todo!()
    }

    fn render_pass(&self, _gpu_state: &GPUState, _view: &TextureView) {
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
        //todo
        ModelComponent { model, instance_id: 0 }
    }
}