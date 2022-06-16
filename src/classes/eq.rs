use std::marker::PhantomData;
use crate::*;

pub struct EqTo<T: Value>(PhantomData<T>);

impl<T: Eq, I: Value<Type=T>> Value for EqTo<I> {
    type Type = Lambda<T, Bool>;
}

pub trait Eq: Type {}