use cgmath::{One, Quaternion};

use functional_game_engine::Application;
use functional_game_engine::asset::AssetStore;
use functional_game_engine::game::entity::{Change, Component, Entity};
use functional_game_engine::game::GameState;
use functional_game_engine::game::transform::{get_pos, Transform2D, Transform3D, TRANSFORM_COMP_NAME};
use functional_game_engine::render::model_render::ModelComponent;
use functional_game_engine::render::sprite_render::SpriteComponent;
use functional_game_engine::util::Either;
use functional_game_engine::util::res::Res;

#[derive(Copy, Clone)]
struct Tag {
    _i: u32,
}

fn setup(game_state: &mut GameState, assets: Res<AssetStore>) {
    println!("hello world!!");

    // loading the assets
    let (cat_sprite, box_model) = {
        let mut ass = assets.write().unwrap();
        (ass.load_material("angry_cat.png"), ass.load_model("Crate1"))
    };

    // initialing the entities
    {
        let mut e1 = Entity::new();
        e1.add_comp(Transform2D {
            pos: [-1., -0.2],
            size: [0.5, 0.5],
            rot: 0.
        });
        e1.add_comp(Tag {
            _i: 10
        });
        e1.add_comp(SpriteComponent::new(cat_sprite.clone()));
    }
    {
        let mut e2 = Entity::new();
        e2.add_comp(Transform2D {
            pos: [-1., -1.],
            size: [1.0, 0.5],
            rot: 1.0
        });
        e2.add_comp(SpriteComponent::new(cat_sprite));
    }
    {
        let mut e3 = game_state.new_entity_mut();
        e3.add_comp(Transform3D {
            pos: [0., 0., 0.],
            size: [1.0, 1.0, 1.0],
            rotation: Quaternion::one(),
        });
        e3.add_comp(ModelComponent::new(box_model));
    }

    game_state.systems.push(|entity| {
        if let Some(Either::This(mut p)) = get_pos(entity.data()) {
            if entity.data().has("tag") {
                if p.pos[0] > 1.0 {
                    p.pos[0] = -1.;
                } else {
                    p.pos[0] += 0.01;
                }
                Some(Change::new(p, TRANSFORM_COMP_NAME))
            } else {
                if p.rot > 2.0 {
                    p.rot = 0.0;
                } else {
                    p.rot += 0.01;
                }
                Some(Change::new(p, TRANSFORM_COMP_NAME))
            }
        } else {
            None
        }
    });

    // example quadratic system:
    /* // spams the console a lot
    game_state.quadratic_systems.push(|entity, other| {
        if let Some(pos1) = entity.data().get::<Transform2D>("pos") {
            if let Some(pos2) = other.data().get::<Transform2D>("pos") {
                if Transform2D::dist(pos1, pos2) <= 1.0 {
                    println!("Entity:{} and Entity:{} are close!", entity.id(), other.id());
                }
            }
        }
        None
    });
    */
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {
    Application::new()
        .with_setup(setup)
        .run();
}
