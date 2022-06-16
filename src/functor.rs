use crate::{Eval, Lambda, Type, Value};

#[allow(dead_code)]
pub type Map<A, B, F, M> = Eval<<<M as Value>::Type as Functor<A>>::MapOn<B, F>, M>;

pub trait Functor<T: Type>: Type {
    type HKT<A: Type>: Type;
    type MapOn<B: Type, F: Value<Type=Lambda<T, B>>>: Value<Type=Lambda<Self, Self::HKT<B>>>;
}