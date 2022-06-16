use std::marker::PhantomData;
use crate::*;

#[derive(Copy, Clone, Default)]
pub struct Maybe<T: Type>(PhantomData<T>);

impl<T: Type> Type for Maybe<T> {}

#[derive(Copy, Clone, Default)]
pub struct Nothing<T: Type>(PhantomData<T>);

#[derive(Copy, Clone, Default)]
pub struct Just<V: Value>(PhantomData<V>);

impl<T: Type> Value for Nothing<T> {
    type Type = Maybe<T>;
}

impl<T: Type, V: Value<Type=T>> Value for Just<V> {
    type Type = Maybe<T>;
}

#[derive(Copy, Clone, Default)]
pub struct Flatten<T: Type>(PhantomData<T>);

impl<T: Type> Value for Flatten<T> {
    type Type = Lambda<Maybe<Maybe<T>>, Maybe<T>>;
}

impl<T: Type> App<Nothing<Maybe<T>>> for Flatten<T> {
    type Result = Nothing<T>;
}

impl<T: Type, I: Value<Type=Maybe<T>>> App<Just<I>> for Flatten<T> {
    type Result = I;
}
// monad
impl<T: Type> Monad for Maybe<T> {
    type Wrapped = T;
    type HKT<A: Type> = Maybe<A>;
}

impl<
    T: Type,
    V: Value<Type=T>
> App<V> for Pure<Maybe<T>> {
    type Result = Just<V>;
}

impl<A: Type, B: Type, F: Value<Type=Lambda<A, Maybe<B>>>> App<Nothing<A>> for BindOn<Maybe<A>, F> {
    type Result = Nothing<B>;
}

impl<
    A: Type,
    B: Type,
    F: Value<Type=Lambda<A, Maybe<B>>> + App<Ia, Result=R>,
    Ia: Value<Type=A>,
    R: Value<Type=Maybe<B>>
> App<Just<Ia>> for BindOn<Maybe<A>, F> {
    type Result = R;
}

// functor
impl<T: Type> Functor<T> for Maybe<T> {
    type HKT<A: Type> = Maybe<A>;
}

impl<A: Type, B: Type, F: Value<Type=Lambda<A, B>>> App<Nothing<A>> for MapOn<Maybe<A>, F> {
    type Result = Nothing<B>;
}

impl<
    A: Type,
    B: Type,
    F: Value<Type=Lambda<A, B>> + App<V, Result=R>,
    V: Value<Type=A>,
    R: Value<Type=B>
> App<Just<V>> for MapOn<Maybe<A>, F> {
    type Result = Just<R>;
}


