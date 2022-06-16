use std::marker::PhantomData;
use crate::*;

#[derive(Copy, Clone, Default)]
pub struct Maybe<T: Type>(PhantomData<T>);

#[derive(Copy, Clone, Default)]
pub struct Nothing<T: Type>(PhantomData<T>);

#[derive(Copy, Clone, Default)]
pub struct Just<V: Value>(PhantomData<V>);

impl<T: Type> Type for Maybe<T> {}

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

#[derive(Copy, Clone, Default)]
pub struct Bind<A: Type, B: Type>(PhantomData<(A, B)>);

impl<A: Type, B: Type> Value for Bind<A, B> {
    type Type = Lambda<
        (Maybe<A>, Lambda<A, Maybe<B>>,),
        Maybe<B>
    >;
}

impl<A: Type, B: Type, F: Value<Type=Lambda<A, Maybe<B>>>> App<(Nothing<A>, F)> for Bind<A, B> {
    type Result = Nothing<B>;
}

impl<
    A: Type,
    B: Type,
    F: Value<Type=Lambda<A, Maybe<B>>> + App<Ia, Result=R>,
    Ia: Value<Type=A>,
    R: Value<Type=Maybe<B>>
> App<(Just<Ia>, F)> for Bind<A, B> {
    type Result = R;
}

pub struct Pure<T: Type>(PhantomData<T>);

impl<T: Type> Value for Pure<T> {
    type Type = Lambda<T, Maybe<T>>;
}

impl<T: Type, V: Value<Type=T>> App<V> for Pure<T> {
    type Result = Just<V>;
}

impl<T: Type> Monad<T> for Maybe<T> {
    type HKT<A: Type> = Maybe<A>;
    type Pure = Pure<T>;
    type Bind<B: Type> = Bind<T, B>;
}

