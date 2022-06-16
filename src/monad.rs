use std::marker::PhantomData;
use crate::*;

pub trait Monad: Type {
    type Wrapped: Type;
    type HKT<A: Type>: Type + Monad<Wrapped=A>;
}

pub type Return<M, V> = Eval<Pure<M>, V>;

#[derive(Copy, Clone, Default)]
pub struct Pure<M: Monad>(PhantomData<M>);

impl<
    MT: Monad,
> Value for Pure<MT> {
    type Type = Lambda<MT::Wrapped, MT>;
}

pub type Bind<M, F> = Eval<BindOn<<M as Value>::Type, F>, M>;
#[derive(Copy, Clone, Default)]
pub struct BindOn<M: Monad, F: Value>(PhantomData<(M, F)>);

impl<
    A: Type, B: Type,
    MB: Type + Monad<Wrapped=B>,
    M: Monad<Wrapped=A, HKT<B> =MB>,
    F: Value<Type=Lambda<A, MB>>
> Value for BindOn<M, F>{
    type Type = Lambda<M, MB>;
}