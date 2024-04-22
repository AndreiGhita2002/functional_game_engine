use functional_game_engine::run;
use functional_game_engine::game::entity::{Change, Component};
use functional_game_engine::game::GameState;
use functional_game_engine::game::transform::Transform2D;
use functional_game_engine::render::asset::AssetsToLoad;
use functional_game_engine::render::sprite::Sprite;

#[derive(Copy, Clone)]
struct Tag {
    _i: u32,
}

fn main() {
    println!("hello world!!");

    let mut game_state = GameState::new();

    {
        let mut e1 = game_state.new_entity_mut();
        e1.mut_data().alloc(Transform2D { pos: [-1., -0.2] }, "pos");
        e1.mut_data().alloc(Tag { _i: 10 }, "tag");
        Sprite::new(0).to_entity(&mut e1);
    }
    {
        let mut e2 = game_state.new_entity_mut();
        e2.mut_data().alloc(Transform2D { pos: [-1., -1.] }, "pos");
        Sprite::new(0).to_entity(&mut e2);
    }
    // {
    //     let mut e3 = game_state.new_entity_mut();
    //     e3.mut_data().alloc(Transform2D { pos: [1., -0.3] }, "pos");
    //     Sprite::new(0).to_entity(&mut e3);
    // }
    // {
    //     let mut e4 = game_state.new_entity_mut();
    //     e4.mut_data().alloc(Transform2D { pos: [-1., -0.5] }, "pos");
    //     Sprite::new(0).to_entity(&mut e4);
    // }

    game_state.systems.push(|entity| {
        if entity.data().has("tag") {
            if let Some(mut p) = entity.data().get::<Transform2D>("pos") {
                if p.pos[0] > 1.0 {
                    p.pos[0] = -1.;
                } else {
                    p.pos[0] += 0.01;
                }
                Some(Change::new(p, "pos"))
            } else {
                None
            }
        } else {
            None
        }
    });

    // game_state.systems.push(|entity| {
    //     if let Some(mut p) = entity.data().get::<Transform2D>("pos") {
    //         if p.pos[0] > 1.0 {
    //             p.pos[0] = -1.;
    //         } else {
    //             p.pos[0] += 0.1;
    //         }
    //         Some(Change::new(p, "pos"))
    //     } else {
    //         None
    //     }
    // });

    let to_load = AssetsToLoad {
        texture_files: vec!["angry_cat.png".to_string()]
    };

    pollster::block_on(run(game_state, to_load));
}
