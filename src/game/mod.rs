use std::collections::HashMap;
use std::fmt;
use std::time::Duration;
use once_cell::sync::Lazy;

use crate::game::component::{Component, ComponentTable};
use crate::game::entity::Entity;
use crate::game::system::System;
use crate::game::transform::Transform2D;
use crate::util::res::Global;

pub mod entity;
pub mod transform;
pub mod component;
pub mod system;

pub struct GameState {
    pub entities: Vec<Entity>,
    pub component_table: ComponentTable,
    next_id: u64
}

pub static GAME_STATE: Lazy<Global<GameState>> = Lazy::new( ||
    Global::new(GameState::new())
);
pub static GAME_SYSTEMS: Lazy<Global<Vec<Box<dyn System + Send + Sync>>>> = Lazy::new(||
    Global::new(Vec::new())
);

/// When and how many times should a System be run?
/// todo: use System Times
pub enum Times {
    Startup,
    SimulationTick,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            entities: Vec::new(),
            // systems that are applied on single entities
            component_table: ComponentTable {
                rows: HashMap::new()
            },
            next_id: 0,
        }
    }

    pub fn print_comps<C: Component + fmt::Display>(&self) {
        let comp_str = C::static_type_identifier();
        println!("Components {}:", comp_str);
        if let Some(row) = self.component_table.rows.get(comp_str) {
            for component in row.iter() {
                if let Ok(c) = component.data.as_type::<C>() {
                    print!("  {c}\n")
                }
            }
        }
    }
}

pub fn add_system(system: Box<dyn System + Send + Sync>) {
    let mut systems = GAME_SYSTEMS.write().unwrap();
    systems.push(system);
}

pub fn game_tick(_delta_t: Duration) {
    let mut game_state = GAME_STATE.write().unwrap();
    let systems = GAME_SYSTEMS.read().unwrap();
    for system in systems.iter() {
        system.execute(&mut game_state)
    }
    game_state.print_comps::<Transform2D>();
}
