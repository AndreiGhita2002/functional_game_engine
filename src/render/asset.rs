use crate::render::model::Material;

pub type MaterialId = usize;

pub struct AssetStore {
    materials: Vec<Material>,
}

impl AssetStore {
    pub fn new() -> Self {
        AssetStore {
            materials: Vec::new()
        }
    }

    pub fn get_material(&self, id: MaterialId) -> Option<&Material> {
        self.materials.get(id)
    }
}