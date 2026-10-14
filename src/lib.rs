use std::marker::PhantomData;

pub mod registry;
pub mod interner;
mod tests;

pub struct Id<T> {
    pub index: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Clone for Id<T> { fn clone(&self) -> Self { *self } }
impl<T> Copy for Id<T> {}
impl<T> PartialEq for Id<T> { fn eq(&self, other: &Self) -> bool { self.index == other.index } }
impl<T> Eq for Id<T> {}
impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.index.hash(state) }
}
impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id({})", self.index)
    }
}