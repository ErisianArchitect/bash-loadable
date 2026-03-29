use std::marker::PhantomData;


pub(crate) mod seal {
    #[allow(unused)]
    pub trait Seal<T> {}
}

/// Marker used for Immutable reference types.
pub struct Immut<'a>(PhantomData<&'a ()>);
impl<'a, T> seal::Seal<T> for Immut<'a> {}

/// Marker for Mutable reference types.
pub struct Mut<'a>(PhantomData<&'a mut ()>);
impl<'a, T> seal::Seal<T> for Mut<'a> {}

/// Marker for Owned types. These types must implement Drop.
pub struct Owned;
impl<T> seal::Seal<T> for Owned {}