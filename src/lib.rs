use std::sync::{Arc, Mutex};
use crate::game::entity::Entity;

mod game;
mod util;

struct GameState {
    entities: Vec<Arc<Mutex<Entity>>>
}
