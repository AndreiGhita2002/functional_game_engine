use std::fmt;
use std::time::Duration;

use crate::game::entity::{Entity, EntityChange};

pub mod entity;
pub mod transform;

type LinearSystem = fn(&Entity) -> Option<Box<dyn EntityChange>>;
type QuadraticSystem = fn(&Entity, &Entity) -> Option<Box<dyn EntityChange>>;

pub struct GameState {
    pub entities: Vec<Entity>,
    pub linear_systems: Vec<LinearSystem>,
    pub quadratic_systems: Vec<QuadraticSystem>,
    next_id: u64
}


/// When and how many times should a System be run?
pub enum Times {
    Startup,
    SimulationTick,
}


impl GameState {
    pub fn new() -> Self {
        GameState {
            entities: Vec::new(),
            // systems that are applied on single entities
            linear_systems: Vec::new(),
            // systems that are applied on pairs of entities
            quadratic_systems: Vec::new(),
            next_id: 0,
        }
    }

    pub fn sim_tick(&mut self, _delta_t: Duration) {
        let mut changes: Vec<(usize, Box<dyn EntityChange>)> = Vec::new();

        for (i, entity) in self.entities.iter().enumerate() {
            // first we apply every linear system to it
            for lin_sys in self.linear_systems.iter() {
                if let Some(change) = lin_sys(entity) {
                    changes.push((i, change));
                }
            }

            // then we loop through every other entity
            for other in self.entities.iter() {
                if entity.id() != other.id() {
                    // apply every quadratic system on this pair
                    for quad_sys in self.quadratic_systems.iter() {
                        if let Some(change) = quad_sys(entity, other) {
                            // changes are only applied to the first entity
                            changes.push((i, change));
                        }
                    }
                }
            }
        }

        // now we apply the changes
        for (i, change) in changes {
            if let Some(entity) = self.entities.get_mut(i) {
                entity.resolve_changes(change);
            }
        }
    }

    #[allow(dead_code)]
    pub fn new_entity(&mut self) -> &Entity {
        self.entities.push(Entity::new(self.next_id));
        self.next_id += 1;
        self.entities.last().unwrap()
    }

    #[allow(dead_code)]
    pub fn new_entity_mut(&mut self) -> &mut Entity {
        self.entities.push(Entity::new(self.next_id));
        self.next_id += 1;
        self.entities.last_mut().unwrap()
    }

    pub fn print_comps<T: fmt::Display + Clone>(&self, comp_label: &str) {
        println!("Components {}:", comp_label);
        for entity in self.entities.iter() {
            print!("{}: ", entity.id());
            let _ = entity.print_comp::<T>(comp_label);
            println!();
        }
    }
}
