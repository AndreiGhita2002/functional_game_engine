use std::intrinsics::transmute;
use std::ptr;

use mem_macros::size_of;

pub mod arena;
pub mod res;

pub enum Either<T1, T2> {
    This(T1),
    That(T2),
}

pub trait Byteable {
    fn from_bytes(bytes: &[u8]) -> Self;

    fn from_vec_bytes(vec: Vec<u8>) -> Self;

    fn into_bytes(self) -> Vec<u8>;
}

impl<T> Byteable for T {
    fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), size_of!(T));
        let p: *const u8 = bytes.as_ptr();
        unsafe {
            ptr::read(transmute::<*const u8, *const T>(p))
        }
    }

    #[inline]
    fn from_vec_bytes(vec: Vec<u8>) -> Self {
        T::from_bytes(vec.as_slice())
    }

    fn into_bytes(self) -> Vec<u8> {
        let p: *const T = &self;
        let slice = unsafe {
            let t_slice = std::slice::from_raw_parts(p, size_of!(T));
            transmute::<&[T], &[u8]>(t_slice)
        };
        Vec::from(slice)
    }
}
