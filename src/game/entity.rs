use crate::game::GAME_STATE;

use crate::game::component::{Component, ComponentHolder};

pub struct Entity {
    id: u64,
}

impl Entity {
    pub fn new() -> &'static Self {
        let mut game = GAME_STATE.lock().unwrap();
        let e = Entity {
            id: game.next_id,
        };
        game.next_id += 1;
        game.entities.push(e);
        game.entities.last().unwrap()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn data(&self) -> Vec<&Box<dyn Component>> {
        let state = GAME_STATE.lock().expect("GAME_STATE mutex is poisoned!");
        state.component_table.entity_components(self.id)
    }

    pub fn mut_data(&self) -> Vec<&mut Box<dyn Component>> {
        let mut state = GAME_STATE.lock().expect("GAME_STATE mutex is poisoned!");
        state.component_table.entity_components_mut(self.id)
    }

    pub fn add_comp<C: Component>(&self, component: C) {
        let mut state = GAME_STATE.lock().expect("GAME_STATE mutex is poisoned!");
        let comp_holder = ComponentHolder {
            entity_id: self.id,
            data: Box::new(component),
        };
        state.component_table.add_comp(comp_holder);
    }
}

//todo are EntityChanges still needed? I don't think so

/*
/// What changes are done to an Entity?
pub trait EntityChange {
    fn arena_insert(self: Box<Self>, arena: &mut ComponentArena) -> anyhow::Result<()>;
}

/// Does a single change to the Entity
pub struct Change<T: Clone> {
    label: String,
    data: Option<T>,
}

impl<T: Clone> EntityChange for Change<T> {
    fn arena_insert(self: Box<Self>, arena: &mut ComponentArena) -> anyhow::Result<()> {
        if self.data.is_none() {
            return Err(anyhow!("Change has no data!"));
        }
        let (label, data) = (self.label, self.data.unwrap());
        arena.insert::<T>(data, &label)
    }
}

impl<T: Clone> Change<T> {
    pub fn new(change: T, label: &str) -> Box<Self> {
        Box::new(Self {
            label: String::from(label),
            data: Some(change),
        })
    }
}
*/
