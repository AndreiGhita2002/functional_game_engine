use std::fmt;
use std::rc::Rc;
use crate::game::entity::{Entity, EntityChange};
use crate::render::asset::AssetStore;

pub mod entity;

type System = fn(&Entity) -> Option<Box<dyn EntityChange>>;

pub struct GameState {
    pub entities: Vec<Entity>,
    pub systems: Vec<System>,
    pub assets: AssetStore,
    next_id: u64
}


/// When and how many times should a System be run?
pub enum Times {
    Startup,
    SimulationTick,
    RenderTick,
}


impl GameState {
    pub fn new() -> Self {
        GameState {
            entities: Vec::new(),
            systems: Vec::new(),
            assets: AssetStore::new(),
            next_id: 0,
        }
    }

    pub fn sim_tick(&mut self) {
        for system in self.systems.iter() {
            for entity in self.entities.iter_mut() {
                if let Some(change) = system(entity) {
                    entity.resolve_changes(change);
                }
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

    pub fn print_comps<T: fmt::Display>(&self, comp_label: &str) {
        println!("Components {}:", comp_label);
        for entity in self.entities.iter() {
            print!("{}: ", entity.id());
            let _ = entity.print_comp::<T>(comp_label);
            println!();
        }
    }
}
