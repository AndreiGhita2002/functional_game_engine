use std::collections::HashMap;
use anyhow::anyhow;

pub struct ComponentTable {
    pub rows: HashMap<String, Vec<ComponentHolder>>
}

pub struct ComponentHolder {
    pub entity_id: u64,
    pub data: Box<dyn Component + Send>,
}

pub trait Component {
    /// Returns the identifier of this type of component.
    /// should be the same as ``instance_type_identifier()``
    fn static_type_identifier() -> &'static str where Self: Sized;

    /// Returns the identifier of this type of component.
    /// should be the same as ``static_type_identifier()``
    fn instance_type_identifier(&self) -> &'static str;
}

impl dyn Component {
    pub fn as_type<T: Component>(&self) -> anyhow::Result<&T> {
        if T::static_type_identifier() != self.instance_type_identifier() {
            Err(anyhow!(
                "Component of {} was wrongly tried to be cast as {}",
                self.instance_type_identifier(),
                T::static_type_identifier()
            ))
        } else {
            // inspired by Any::downcast_ref_unchecked()
            Ok(unsafe {
                &*(self as *const dyn Component as *const T)
            })
        }
    }

    pub fn as_mut_type<T: Component>(&mut self) -> anyhow::Result<&mut T> {
        if T::static_type_identifier() != self.instance_type_identifier() {
            Err(anyhow!(
                "Component of {} was wrongly tried to be cast as {}",
                self.instance_type_identifier(),
                T::static_type_identifier()
            ))
        } else {
            // inspired by Any::downcast_mut_unchecked()
            Ok(unsafe {
                &mut *(self as *mut dyn Component as *mut T)
            })
        }
    }
}

macro_rules! impl_component {
    ($t:ty, $name:literal) => {
        impl Component for $t {
            fn static_type_identifier() -> &'static str where Self: Sized {
                $name
            }

            fn instance_type_identifier(&self) -> &'static str {
                $name
            }
        }
    };
    ($t:ty) => {
        impl Component for $t {
            fn static_type_identifier() -> &'static str where Self: Sized {
                stringify!($t)
            }

            fn instance_type_identifier(&self) -> &'static str {
                stringify!($t)
            }
        }
    };
}
pub(crate) use impl_component;

impl ComponentTable {
    pub fn entity_components(&self, entity_id: u64) -> Vec<&Box<dyn Component>> {
        let mut out = Vec::new();
        for (_, component_row) in self.rows.iter() {
            for comp_holder in component_row.iter() {
                if comp_holder.entity_id == entity_id {
                    out.push(&comp_holder.data)
                }
            }
        }
        out
    }

    pub fn entity_components_mut(&mut self, entity_id: u64) -> Vec<&mut Box<dyn Component>> {
        let mut out = Vec::new();
        for (_, mut component_row) in self.rows.iter_mut() {
            for mut comp_holder in component_row.iter_mut() {
                if comp_holder.entity_id == entity_id {
                    out.push(&mut comp_holder.data)
                }
            }
        }
        out
    }

    pub fn insert_component_holder(&mut self, component_holder: ComponentHolder) {
        let comp_name = component_holder.data.instance_type_identifier();
        if let Some(mut row) = self.rows.get(comp_name) {
            row.push(component_holder);
        } else {
            self.rows.insert(comp_name.to_string(), vec![component_holder]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::component::Component;

    struct Comp1 {
        x: i32,
    }
    struct Comp2 {
        a: i32,
        b: i32,
    }
    impl_component!(Comp1);
    impl_component!(Comp2);

    #[test]
    fn test_as_type() {
        let comp1_box: Box<dyn Component> = Box::new(Comp1{
            x: 10
        });
        let comp1 = comp1_box.as_type::<Comp1>();
        assert!(comp1.is_ok(),
                "Component::as_type failed: returned None when typecast should have been valid!");
        assert_eq!(comp1.unwrap().x, 10,
                   "Component::as_type failed: data inside the component was corrupted!");
        let error = comp1_box.as_type::<Comp2>();
        assert!(error.is_err(),
                "Component::as_type failed: function did an invalid cast!");
    }

    #[test]
    fn test_as_mut_type() {
        let mut comp2_box: Box<dyn Component> = Box::new(Comp2{
            a: 5,
            b: 9,
        });
        {
            let comp2 = comp2_box.as_mut_type::<Comp2>();
            assert!(comp2.is_ok(),
                    "Component::as_mut_type failed: returned None when typecast should have been valid!");

            let c = comp2.unwrap();
            c.a = 10;
            c.b = c.a + 5;
        }
        assert_eq!(comp2_box.as_mut_type::<Comp2>().unwrap().a, 10,
                   "Component::as_mut_type failed: data inside the component was corrupted!");
        assert_eq!(comp2_box.as_mut_type::<Comp2>().unwrap().b, 15,
                   "Component::as_mut_type failed: data inside the component was corrupted!");

        let error = comp2_box.as_mut_type::<Comp1>();
        assert!(error.is_err(),
                "Component::as_mut_type failed: function did an invalid cast!");
    }
}