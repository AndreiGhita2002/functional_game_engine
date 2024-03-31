use std::fmt;
use functional_game_engine::run;
use functional_game_engine::game::entity::Change;
use functional_game_engine::game::GameState;


fn main() {
    println!("hello world!!");

    let mut game_state = GameState::new();

    {
        let e1 = game_state.new_entity_mut();
        e1.mut_data().alloc(FunValue { val: 10 }, "fun");
    }
    {
        let e2 = game_state.new_entity_mut();
        e2.mut_data().alloc(FunValue { val: -6 }, "fun");
    }

    game_state.systems.push(|entity| {
        if let Some(mut fun) = entity.data().get::<FunValue>("fun") {
            fun.val += 1;
            Some(Change::new(fun, "fun"))
        } else {
            None
        }
    });

    // for i in 0..4 {
    //     println!(">Tick {}:", i);
    //     game_state.sim_tick();
    //     game_state.print_comps::<FunValue>("fun");
    // }

    pollster::block_on(run(game_state));
}

struct FunValue {
    val: i32,
}

impl fmt::Display for FunValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}