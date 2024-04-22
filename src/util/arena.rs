use std::collections::HashMap;
use std::ptr;
use std::mem::transmute;
use anyhow::anyhow;
use mem_macros::size_of;

// todo make thread safe
pub struct Arena {
    data: Vec<u8>,
    labels: HashMap<String, (usize, usize)>,
}

impl Arena {
    pub fn new() -> Self {
        Arena {
            data: Vec::new(),
            labels: HashMap::new(),
        }
    }

    pub fn get<T>(&self, label: &str) -> Option<T> {
        let (start, end) = self.labels.get(label)?;
        let slice = self.data.as_slice().get(*start..*end)?.as_ptr();
        Some(unsafe {
            ptr::read(transmute::<*const u8, *const T>(slice))
        })
    }

    pub fn get_bytes(&self, label: &str) -> Option<&[u8]> {
        let (start, end) = self.labels.get(label)?;
        self.data.as_slice().get(*start..*end)
    }

    pub fn get_length(&self, label: &str) -> Option<usize> {
        let (start, end) = self.labels.get(label)?;
        return Some(end - start);
    }

    //todo test this
    #[allow(dead_code)]
    pub fn get_mut_bytes(&mut self, label: &str) -> Option<&mut [u8]> {
        let (start, end) = self.labels.get(label)?;
        self.data.as_mut_slice().get_mut(*start..*end)
    }

    pub fn alloc_raw(&mut self, data: &[u8], label: &str) {
        let start = self.data.len();
        let end = start + data.len();
        self.labels.insert(label.to_string(), (start, end));
        self.data.extend_from_slice(data);
    }

    pub fn alloc<T>(&mut self, data: T, label: &str) {
        let p: *const T = &data;
        let bytes: &[u8] = unsafe {
            let t_slice = std::slice::from_raw_parts(p, size_of!(T));
            transmute::<&[T], &[u8]>(t_slice)
        };
        self.alloc_raw(&bytes, label)
    }

    pub fn insert_raw(&mut self, data: &[u8], label: &str) -> anyhow::Result<()> {
        //todo add proper errors to this function, rather than anyhow
        let (start, end) = match self.labels.get(label) {
            None => return Err(anyhow!("label does not exist!")),
            Some(r) => *r,
        };
        let l = end - start;
        if l > data.len() {
            return Err(anyhow!("data is longer than the destination!"));
        }
        //coping data into the vector:
        for i in 0..l {
            self.data.insert(start + i, data[i]);
        }
        Ok(())
    }

    pub fn insert<T>(&mut self, data: T, label: &str) -> anyhow::Result<()> {
        let p: *const T = &data;
        let bytes: &[u8] = unsafe {
            let t_slice = std::slice::from_raw_parts(p, size_of!(T));
            transmute::<&[T], &[u8]>(t_slice)
        };
        self.insert_raw(bytes, label)
    }

    pub fn has(&self, label: &str) -> bool {
        self.labels.get(label).is_some()
    }

    pub fn labels(&self) -> Vec<String> {
        let mut out = Vec::new();
        for key in self.labels.keys() {
            out.push(key.clone());
        }
        return out;
    }
}

#[cfg(test)]
mod tests {
    use bytemuck::{Pod, Zeroable};
    use crate::util::arena::*;

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Zeroable, Pod)]
    struct TestStruct1 {
        i: i32,
        u: u32,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Zeroable, Pod)]
    struct TestStruct2 {
        f: f32,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Zeroable, Pod)]
    struct TestStruct3 {
        x: i32,
        arr: [f32; 2],
    }

    #[test]
    fn arena_raw_alloc() {
        let mut arena = Arena::new();

        let data = [1u8, 2u8, 3u8, 4u8];  // 1,2,3,4

        arena.alloc_raw(&data, "test");

        assert_eq!(arena.get_bytes("test"), Some(&data[..]))
    }

    #[test]
    fn arena_byte_access() {
        let mut arena = Arena::new();

        // allocation
        {
            let s1 = TestStruct1 { i: -28, u: 74 };
            let s2 = TestStruct2 { f: 0.2345 };
            let s3 = TestStruct3 { x: 10, arr: [1.1, 2.2] };

            arena.alloc(s1, "s1");
            arena.alloc(s2, "s2");
            arena.alloc(s3, "s3");
        }
        // access
        {
            let s1: TestStruct1 = bytemuck::cast_slice::<u8, TestStruct1>(arena.get_bytes("s1").unwrap())[0];
            assert_eq!(s1.i, -28);
            assert_eq!(s1.u, 74);
            let s2: TestStruct2 = bytemuck::cast_slice::<u8, TestStruct2>(arena.get_bytes("s2").unwrap())[0];
            assert_eq!(s2.f, 0.2345);
            let s3: TestStruct3 = bytemuck::cast_slice::<u8, TestStruct3>(arena.get_bytes("s3").unwrap())[0];
            assert_eq!(s3.x, 10);
            assert_eq!(s3.arr, [1.1, 2.2]);
            // testing that None is returned
            let none: Option<&[u8]> = arena.get_bytes("s0");
            assert!(none.is_none());
        }
    }

    #[test]
    fn arena_generic_access() {
        let mut arena = Arena::new();

        // allocation
        {
            let s1 = TestStruct1 { i: -28, u: 74 };
            let s2 = TestStruct2 { f: 0.2345 };
            let s3 = TestStruct3 { x: 10, arr: [1.1, 2.2] };

            arena.alloc(s1, "s1");
            arena.alloc(s2, "s2");
            arena.alloc(s3, "s3");
        }
        // access
        {
            let s1: TestStruct1 = arena.get("s1").unwrap();
            assert_eq!(s1.i, -28);
            assert_eq!(s1.u, 74);
            let s2: TestStruct2 = arena.get("s2").unwrap();
            assert_eq!(s2.f, 0.2345);
            let s3: TestStruct3 = arena.get("s3").unwrap();
            assert_eq!(s3.x, 10);
            assert_eq!(s3.arr, [1.1, 2.2]);
            // testing that None is returned
            let none: Option<TestStruct1> = arena.get("s0");
            assert!(none.is_none());
        }
    }

    #[test]
    fn arena_insert() {
        let mut arena = Arena::new();

        // allocation
        {
            let s1 = TestStruct1 { i: -28, u: 74 };
            let s2 = TestStruct2 { f: 0.2345 };
            let s3 = TestStruct3 { x: 10, arr: [1.1, 2.2] };

            arena.alloc(s1, "s1");
            arena.alloc(s2, "s2");
            arena.alloc(s3, "s3");
        }
        // access
        {
            let mut s1: TestStruct1 = arena.get("s1").unwrap();
            s1.i = 2;
            s1.u = 1;
            arena.insert(s1, "s1").unwrap();
            s1 = arena.get("s1").unwrap();
            assert_eq!(s1.i, 2);
            assert_eq!(s1.u, 1);

            let s2: TestStruct2 = arena.get("s2").unwrap();
            assert!(arena.insert(s2, "s1").is_err());
        }
    }

    #[test]
    fn double_get() {
        let mut arena = Arena::new();

        // allocation
        {
            arena.alloc(TestStruct1 { i: -28, u: 74 }, "s1");
        }
        // access
        {
            let mut s1: TestStruct1 = arena.get("s1").unwrap();
            s1.i = 2;
            arena.insert(s1, "s1").unwrap();
        }
        // second access
        {
            let s1: TestStruct1 = arena.get("s1").unwrap();
            assert_eq!(s1.i, 2);
            assert_eq!(s1.u, 74);
        }
    }
}
