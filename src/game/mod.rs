use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;
use std::time::Duration;

use crate::game::component::ComponentTable;
use crate::game::entity::Entity;
use crate::game::system::System;

pub mod entity;
pub mod transform;
pub mod component;
pub mod system;

pub struct GameState {
    pub entities: Vec<Entity>,
    pub component_table: ComponentTable,
    pub systems: Vec<Box<dyn System>>,
    next_id: u64
}

pub static GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new());

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
            systems: Vec::new(),
            next_id: 0,
        }
    }

    pub fn sim_tick(&mut self, _delta_t: Duration) {
        // let mut changes: Vec<(usize, Box<dyn EntityChange>)> = Vec::new();
        //
        // for (i, entity) in self.entities.iter().enumerate() {
        //     // first we apply every linear system to it
        //     for lin_sys in self.linear_systems.iter() {
        //         if let Some(change) = lin_sys(entity) {
        //             changes.push((i, change));
        //         }
        //     }
        //
        //     // then we loop through every other entity
        //     for other in self.entities.iter() {
        //         if entity.id() != other.id() {
        //             // apply every quadratic system on this pair
        //             for quad_sys in self.quadratic_systems.iter() {
        //                 if let Some(change) = quad_sys(entity, other) {
        //                     // changes are only applied to the first entity
        //                     changes.push((i, change));
        //                 }
        //             }
        //         }
        //     }
        // }
        //
        // // now we apply the changes
        // for (i, change) in changes {
        //     if let Some(entity) = self.entities.get_mut(i) {
        //         entity.resolve_changes(change);
        //     }
        // }
    }

    pub fn print_comps<T: fmt::Display + Clone>(&self, comp_label: &str) {
        println!("print_comps({comp_label}): needs to be updated!!")
        //todo

        // println!("Components {}:", comp_label);
        // for entity in self.entities.iter() {
        //     print!("{}: ", entity.id());
        //     let _ = entity.print_comp::<T>(comp_label);
        //     println!();
        // }
    }
}
