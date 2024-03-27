use std::{fmt, mem};
use anyhow::anyhow;
use crate::util::arena::Arena;

pub struct Entity {
    id: u64,
    data: Arena,
}

impl Entity {
    pub fn new(id: u64) -> Self {
        Entity {
            id,
            data: Arena::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn data(&self) -> &Arena {
        &self.data
    }

    pub fn mut_data(&mut self) -> &mut Arena {
        &mut self.data
    }

    pub fn resolve_changes(&mut self, mut changes: Box<dyn EntityChange>) {
        // todo error catching for resolve_changes
        changes.arena_insert(self.mut_data()).unwrap();
    }

    pub fn print_comp<T: fmt::Display>(&self, label: &str) -> Option<()> {
        let comp: T = self.data.get(label)?;
        print!("{}", comp);
        Some(())
    }
}


/// What changes are done to an Entity?
pub trait EntityChange {
    fn arena_insert(&mut self, arena: &mut Arena)  -> anyhow::Result<()>;
}

/// Does a single change to the Entity
pub struct Change<T> {
    label: String,
    data: Option<T>,
}

impl<T> EntityChange for Change<T> {
    fn arena_insert(&mut self, arena: &mut Arena) -> anyhow::Result<()> {
        if self.data.is_none() {
            return Err(anyhow!("Change has no data!"));
        }
        let mut data: Option<T> = None;
        mem::swap(&mut self.data, &mut data);
        arena.insert(data.unwrap(), &self.label)
    }
}

impl<T> Change<T> {
    pub fn new(change: T, label: &str) -> Box<Self> {
        Box::new(Self {
            label: String::from(label),
            data: Some(change),
        })
    }
}
