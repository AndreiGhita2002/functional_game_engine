use std::rc::Rc;
use wgpu::RenderPass;
use crate::game::entity::{Component, Entity};
use crate::render::asset::{AssetStore, MaterialId};


pub fn render_nothing(_entity: &Entity, _store: &AssetStore, _pass: &mut RenderPass) {}

pub fn render_sprite(entity: &Entity, store: &AssetStore, pass: &mut RenderPass) {
    let sprite: Sprite = entity.data().get("sprite").unwrap();
    if let Some(material) = store.get_material(sprite.material_id) {
        pass.set_bind_group(0, &material.bind_group, &[]);
        // todo: pass in a square
        // pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        // pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..4, 0, 0..1);
    } else {
        println!("[ERR] Material with id:{} for Entity:{} not found", sprite.material_id, entity.id());
    }


    // for mesh in &model.meshes {
    //     let material = &model.materials[mesh.material];
    //     render_pass.set_bind_group(0, &material.bind_group, &[]);
    //     render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    //     render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    //     render_pass.draw_indexed(0..mesh.num_elements, 0, instances.clone());
    // }
}

struct Sprite {
    material_id: MaterialId,
}

impl Component for Sprite {
    fn to_entity(self, entity: &mut Entity) {
        entity.set_render_fn(render_sprite);
        entity.mut_data().alloc(self, "sprite");
    }
}

