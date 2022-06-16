use std::marker::PhantomData;
use crate::*;

// def
#[derive(Copy, Clone, Default)]
pub struct True;

#[derive(Copy, Clone, Default)]
pub struct False;

#[derive(Copy, Clone, Default)]
pub struct Bool;

impl Type for Bool {}

impl Eq for Bool {}

impl Value for True {
    type Type = Bool;
}

impl Value for False {
    type Type = Bool;
}

// eq
impl App<True> for EqTo<True> {
    type Result = True;
}

impl App<False> for EqTo<False> {
    type Result = True;
}

impl App<True> for EqTo<False> {
    type Result = False;
}

impl App<False> for EqTo<True> {
    type Result = False;
}

// neg
#[derive(Copy, Clone, Default)]
pub struct Neg;

impl Value for Neg {
    type Type = Lambda<Bool, Bool>;
}

impl App<True> for Neg {
    type Result = False;
}

impl App<False> for Neg {
    type Result = True;
}

// lambda branch
#[derive(Copy, Clone, Default)]
pub struct LambdaBranch<Fa: Value, Pa: Value, Fb: Value, Pb: Value>(PhantomData<(Fa, Pa, Fb, Pb)>);

impl<
    T: Type, A: Type, B: Type,
    Fa: Value<Type=Lambda<A, T>>, Pa: Value<Type=A>,
    Fb: Value<Type=Lambda<B, T>>, Pb: Value<Type=B>
> Value for LambdaBranch<Fa, Pa, Fb, Pb> {
    type Type = Lambda<Bool, T>;
}

impl<
    T: Type, A: Type, B: Type,
    Fa: Value<Type=Lambda<A, T>> + App<Pa, Result=R>,
    Pa: Value<Type=A>,
    Fb: Value<Type=Lambda<B, T>>, Pb: Value<Type=B>,
    R: Value<Type=T>,
> App<True> for LambdaBranch<Fa, Pa, Fb, Pb> {
    type Result = R;
}

impl<
    T: Type, A: Type, B: Type,
    Fa: Value<Type=Lambda<A, T>>, Pa: Value<Type=A>,
    Fb: Value<Type=Lambda<B, T>> + App<Pb, Result=R>,
    Pb: Value<Type=B> ,
    R: Value<Type=T>,
> App<False> for LambdaBranch<Fa, Pa, Fb, Pb> {
    type Result = R;
}

// or
pub type Or<A, B> = Eval<OrOn<A>, B>;

pub struct OrOn<T: Value<Type=Bool>>(PhantomData<T>);

impl<T: Value<Type=Bool>> Value for OrOn<T> {
    type Type = Lambda<Bool, Bool>;
}

impl<T: Value<Type=Bool>> App<True> for OrOn<T> {
    type Result = True;
}

impl App<False> for OrOn<True> {
    type Result = True;
}

impl App<False> for OrOn<False> {
    type Result = False;
}

#[derive(Copy, Clone, Default)]
pub struct WhenMaybe<V: Value>(PhantomData<V>);

impl<T: Type, V: Value<Type=T>> Value for WhenMaybe<V> {
    type Type = Lambda<Bool, Maybe<T>>;
}

impl<T: Type, V: Value<Type=T>> App<True> for WhenMaybe<V> {
    type Result = Just<V>;
}

impl<T: Type, V: Value<Type=T>> App<False> for WhenMaybe<V> {
    type Result = Nothing<T>;
}

#[derive(Copy, Clone, Default)]
pub struct WhenMaybeF<V: Value>(PhantomData<V>);

impl<T: Type, V: Value<Type=T>> Value for WhenMaybeF<V> {
    type Type = Lambda<Bool, Maybe<T>>;
}

impl<T: Type, V: Value<Type=T>> App<False> for WhenMaybeF<V> {
    type Result = Just<V>;
}

impl<T: Type, V: Value<Type=T>> App<True> for WhenMaybeF<V> {
    type Result = Nothing<T>;
}


