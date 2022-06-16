use crate::*;

pub trait Monad<T: Type>: Type {
    type HKT<A: Type>: Type;
    type Pure: Value<Type=Lambda<T, Self::HKT<T>>>;
    type Bind<B: Type>: Value<Type=Lambda<
        (Self, Lambda<T, Self::HKT<B>>),
        Self::HKT<B>
    >>;
}