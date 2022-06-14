use std::marker::PhantomData;
use crate::*;


#[derive(Copy, Clone, Default)]
pub struct Lambda<A: Type, B: Type>(PhantomData<(A, B)>);

impl<A: Type, B: Type> Type for Lambda<A, B> {}

pub trait App<Ia: Value>: Value<Type=Lambda<<Ia as Value>::Type, <Self::Result as Value>::Type>> {
    type Result: Value;
}

impl Type for () {}
impl Value for () {
    type Type = ();
}

pub type Eval<F, P> = <F as App<P>>::Result;

pub struct Id<T: Type>(PhantomData<T>);

impl<T: Type> Value for Id<T> {
    type Type = Lambda<T, T>;
}

impl<T: Type, I: Value<Type=T>> App<I> for Id<T> {
    type Result = I;
}

pub struct Compose<F: Value, G: Value>(PhantomData<(F, G)>);

impl<A: Type, B: Type, C: Type, F: Value<Type=Lambda<A, B>>, G: Value<Type=Lambda<B, C>>> Value for Compose<F, G> {
    type Type = Lambda<A, C>;
}

impl<
    A: Type, B: Type, C: Type,
    F: Value<Type=Lambda<A, B>>, G: Value<Type=Lambda<B, C>>,
    Ia: Value<Type=A>, Ib: Value<Type=B>, Ic: Value<Type=C>
> App<Ia> for Compose<F, G>
    where
        F: App<Ia, Result=Ib>,
        G: App<Ib, Result=Ic>
{
    type Result = Ic;
}