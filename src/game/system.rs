use crate::game::component::Component;
use crate::game::entity::Entity;
use crate::game::GameState;

pub trait System {
    fn execute(&self, game_state: &mut GameState);
}

pub struct OneComponentSystem<C> where C: Component {
    func: fn(&mut C)
}

impl<C: Component> OneComponentSystem<C> {
    pub fn new(func: fn(&mut C)) -> Box<Self> {
        Box::new(OneComponentSystem{func})
    }
}

impl<C: Component> System for OneComponentSystem<C> {
    fn execute(&self, game_state: &mut GameState) {
        let comp_name = C::static_type_identifier();
        let comp_iter = game_state.component_table
            .rows.get_mut(comp_name)
            .expect(&format!("No components of type: {comp_name}"))
            .iter_mut();
        for comp_holder in comp_iter {
            // this should always be true; maybe remove the if?
            if let Ok(comp) = comp_holder.data.as_mut_type::<C>() {
                (self.func)(comp)
            }
        }
    }
}

//todo implement support for this type of system
/*
impl<C1, C2: Component> System for fn(&mut C1, &mut C2) {
    fn execute(&self, game_state: &mut GameState) {
        let comp_name1 = C1::static_type_identifier();
        let comp_name2 = C2::static_type_identifier();
        let _ = game_state.component_table
            .rows.get(comp_name1)
            .expect(&format!("No components of type: {comp_name}"))
            .iter_mut()
            .map(|comp_holder| {
                if let Ok(mut comp) = comp_holder.data.as_type::<C1>() {
                    self(&mut comp)
                }
            });
    }
}*/

impl System for fn(&Entity) {
    fn execute(&self, game_state: &mut GameState) {
        let _ = game_state.entities
            .iter()
            .map(self);
    }
}