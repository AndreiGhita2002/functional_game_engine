use crate::game::component::Component;
use crate::game::entity::Entity;
use crate::game::GameState;

pub trait System {
    fn execute(&self, game_state: &mut GameState);
}

impl<C: Component> System for fn(&mut C) {
    fn execute(&self, game_state: &mut GameState) {
        let comp_name = C::static_type_identifier();
        let _ = game_state.component_table
            .rows.get(comp_name)
            .expect(&format!("No components of type: {comp_name}"))
            .iter_mut()
            .map(|comp_holder| self(&mut comp_holder.data));
    }
}

impl System for fn(&Entity) {
    fn execute(&self, game_state: &mut GameState) {
        let _ = game_state.entities
            .iter()
            .map(self);
    }
}