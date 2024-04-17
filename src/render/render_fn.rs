use wgpu::{RenderBundle, RenderBundleDescriptor, RenderBundleEncoder};
use crate::game::entity::{Component, Entity};
use crate::render::asset::{AssetStore, MaterialId};
use crate::util::res::Res;


pub fn render_sprite(
    entity: &Entity,
    store: Res<AssetStore>,
    mut encoder: RenderBundleEncoder
) -> RenderBundle {
    let sprite: Sprite = entity.data().get("sprite")
        .expect("render_sprite was called on an Entity without a \"sprite\" component!");
    if let Some(material) = store.get_material(sprite.material_id) {
        // passing in the texture
        encoder.set_bind_group(0, &material.bind_group, &[]);
        // passing in the instance
        encoder.set_vertex_buffer(0, store.instance_buffer_2d_slice(..));
        // drawing
        let i = sprite.instance_id..(sprite.instance_id+1);
        encoder.draw(0..4, i);
        // output the bundle
        encoder.finish(&RenderBundleDescriptor {
            label: Some("sprite bundle"),
        })
    } else {
        println!("[ERR] Material with id:{} for Entity:{} not found", sprite.material_id, entity.id());
        encoder.finish(&RenderBundleDescriptor {
            label: Some("nothing bundle"),
        })
    }
}

struct Sprite {
    material_id: MaterialId,
    instance_id: u32,
}

impl Component for Sprite {
    fn to_entity(mut self, entity: &mut Entity) {
        //todo THIS IS VERY BAD!!
        // make some kinda entity id to instance id mapping in AssetStore
        self.instance_id = entity.id() as u32;
        entity.set_render_fn(render_sprite);
        entity.mut_data().alloc(self, "sprite");
    }
}
