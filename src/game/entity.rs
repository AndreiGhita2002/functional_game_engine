use crate::game::component::{Component, ComponentHolder};
use crate::game::GameState;

#[derive(Copy, Clone)]
pub struct Entity {
    id: u64,
}

impl Entity {
    pub fn new(state: &mut GameState) -> Self {
        let e = Entity {
            id: state.next_id,
        };
        state.next_id += 1;
        state.entities.push(e.clone());
        e
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn data<'a>(
        &self,
        state: &'a GameState
    ) -> Vec<&'a Box<dyn Component>> {
        state.component_table.entity_components(self.id)
    }

    pub fn mut_data<'a>(
        &self,
        state: &'a mut GameState
    ) -> Vec<&'a mut Box<dyn Component>> {
        state.component_table.entity_components_mut(self.id)
    }

    pub fn add_comp<C: Component + 'static>(
        &self,
        component: C,
        state: &mut GameState
    ) {
        let comp_holder = ComponentHolder {
            entity_id: self.id,
            data: Box::new(component),
        };
        state.component_table.insert_component_holder(comp_holder);
    }

    pub fn get_comp<'a, C: Component>(
        &self,
        state: &'a GameState
    ) -> Option<&'a C> {
        for comp in self.data(state) {
            if comp.instance_type_identifier() == C::static_type_identifier() {
                return Some(comp.as_type::<C>().unwrap());
            }
        }
        None
    }

    pub fn get_mut_comp<'a, C: Component>(
        &self,
        state: &'a mut GameState
    ) -> Option<&'a mut C> {
        for comp in self.mut_data(state) {
            if comp.instance_type_identifier() == C::static_type_identifier() {
                return Some(comp.as_mut_type::<C>().unwrap());
            }
        }
        None
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
