use std::{fmt, mem};
use std::rc::Rc;
use anyhow::anyhow;
use wgpu::RenderPass;
use crate::render::asset::AssetStore;
use crate::render::render_fn::render_nothing;
use crate::render::RenderFn;
use crate::util::arena::Arena;

pub struct Entity {
    id: u64,
    data: Arena,
    render_fn: RenderFn,
}

impl Entity {
    pub fn new(id: u64) -> Self {
        Entity {
            id,
            data: Arena::new(),
            render_fn: render_nothing,
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

    pub fn render(&self, asset_store: &AssetStore, render_pass: &mut RenderPass) {
        (self.render_fn)(self, asset_store, render_pass);
    }

    pub fn set_render_fn(&mut self, render_fn: RenderFn) {
        self.render_fn = render_fn;
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

pub trait Component {
    fn to_entity(self, entity: &mut Entity);
}
