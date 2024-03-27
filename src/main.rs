use std::fmt;
use std::fmt::Formatter;
use functional_game_engine::{GameState, GPUState};
use functional_game_engine::game::entity::{Change, NoChange};


fn main() {
    println!("hello world!!");

    let mut game_state = GameState::new();
    let _gpu_state = GPUState {};

    {
        let mut e1 = game_state.new_entity_mut();
        e1.mut_data().alloc(FunValue { val: 10 }, "fun");
    }
    {
        let mut e2 = game_state.new_entity_mut();
        e2.mut_data().alloc(FunValue { val: -6 }, "fun");
    }

    game_state.systems.push(|entity| {
        if let Some(mut fun) = entity.data().get::<FunValue>("fun") {
            fun.val += 1;
            Change::new(fun, "fun")
        } else {
            NoChange::new()
        }
    });

    for i in 0..4 {
        println!(">Tick {}:", i);
        game_state.sim_tick();
        game_state.print_comps::<FunValue>("fun");
    }
}

struct FunValue {
    val: i32,
}

impl fmt::Display for FunValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}