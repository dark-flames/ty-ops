use std::marker::PhantomData;
use crate::*;
use crate::classes::*;


#[derive(Copy, Clone, Default)]
pub struct Z;

#[derive(Copy, Clone, Default)]
pub struct S<T> (PhantomData<T>);

#[derive(Copy, Clone, Default)]
pub struct Nat;

impl Type for Nat {}

impl Eq for Nat {}

impl Value for Z {
    type Type = Nat;
}

impl<T: Value<Type=Nat>> Value for S<T> {
    type Type = Nat;
}

// eq

impl App<Z> for EqTo<Z> {
    type Result = True;
}

impl<T: Value<Type=Nat>> App<Z> for EqTo<S<T>> {
    type Result = False;
}

impl<T: Value<Type=Nat>> App<S<T>> for EqTo<Z> {
    type Result = False;
}

impl<L: Value<Type=Nat>, R: Value<Type=Nat>, Eq: Value<Type=Bool>> App<S<R>> for EqTo<S<L>>
    where EqTo<L>: App<R, Result=Eq>
{
    type Result = Eq;
}


// Add
pub type Add<A, B> = Eval<AddOn<A>, B>;
#[derive(Copy, Clone, Default)]
pub struct AddOn<A: Value<Type=Nat>>(PhantomData<A>);

impl<A: Value<Type=Nat>> Value for AddOn<A> {
    type Type = Lambda<Nat, Nat>;
}

impl<A: Value<Type=Nat>> App<Z> for AddOn<A> {
    type Result = A;
}

impl<A: Value<Type=Nat>, B: Value<Type=Nat>, R: Value<Type=Nat>> App<S<B>> for AddOn<A>
    where
        AddOn<A>: Value<Type=Lambda<Nat, Nat>> + App<B, Result=R>
{
    type Result = S<R>;
}
