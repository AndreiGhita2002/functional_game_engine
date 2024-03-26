use bumpalo::Bump;

pub struct Entity {
    id: u64,
    data: Bump,
}

impl Entity {
    pub fn new(id: u64) -> Self {
        Entity {
            id,
            data: Bump::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::entity::Entity;

    struct DogComp {
        fluff: f32,
        age: u32,
    }

    struct CatComp {
        cute: i32,
        mean: i32,
        age: u32,
    }

    #[test]
    fn add_comp_to_entity() {
        let dog_cat_entity = Entity::new(0);

        let dog_comp = dog_cat_entity.data.alloc(DogComp {
            fluff: 0.5,
            age: 7,
        });

        let cat_comp = dog_cat_entity.data.alloc(CatComp {
            cute: 10,
            mean: 5,
            age: 7,
        });

        unsafe {
            for pointer in dog_cat_entity.data.iter_allocated_chunks_raw() {
                println!("{:?} {}", pointer.0, pointer.1)
            }
        }
    }
}