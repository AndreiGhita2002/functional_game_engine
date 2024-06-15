pub mod arena;
pub mod res;
mod anything;

pub enum Either<T1, T2> {
    This(T1),
    That(T2),
}