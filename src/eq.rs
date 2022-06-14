use std::marker::PhantomData;
use crate::*;

pub struct EqTo<T: Value>(PhantomData<T>);

impl<T: Type, I: Value<Type=T>> Value for EqTo<I> {
    type Type = Lambda<T, Bool>;
}

pub struct NotEqTo<T: Value>(PhantomData<T>);

impl<T: Type, I: Value<Type=T>> Value for NotEqTo<I> {
    type Type = Lambda<T, Bool>;
}

impl<T: Type, L: Value<Type=T>, R: Value<Type=T>, Eq: Value<Type=Bool>, Neq: Value<Type=Bool>> App<R> for NotEqTo<L>
where
    EqTo<L>: App<R, Result=Eq>,
    Neg: App<Eq, Result=Neq>
{
    type Result = Neq;
}
