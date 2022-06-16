use std::marker::PhantomData;
use crate::{Eval, Lambda, Type, Value};

pub trait Functor<T: Type>: Type {
    type HKT<A: Type>: Type;
}

pub type Map<F, Fun> = Eval<MapOn<<Fun as Value>::Type, F>, Fun>;

pub struct MapOn<T: Type, F: Value>(PhantomData<(T, F)>);

impl<
    A: Type, B: Type,
    T: Type + Functor<A>,
    F: Value<Type = Lambda<A, B>>
> Value for MapOn<T, F> {
    type Type = Lambda<T, T::HKT<B>>;
}

