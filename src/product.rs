use std::marker::PhantomData;
use crate::*;

impl<A: Type, B: Type> Type for (A, B) {}

impl<TA: Type, TB: Type, A: Value<Type=TA>, B: Value<Type=TB>> Value for (A, B) {
    type Type = (TA, TB);
}

pub struct Fst<A: Type, B: Type>(PhantomData<(A, B)>);
pub struct Snd<A: Type, B: Type>(PhantomData<(A, B)>);

impl<A: Type, B: Type> Value for Fst<A, B> {
    type Type=Lambda<(A, B), A>;
}

impl<A: Type, B: Type, Ia: Value<Type=A>, Ib: Value<Type=B>> App<(Ia, Ib)> for Fst<A, B>
{
    type Result = Ia;
}

impl<A: Type, B: Type> Value for Snd<A, B> {
    type Type=Lambda<(A, B), B>;
}

impl<A: Type,  B: Type, Ia: Value<Type=A>, Ib: Value<Type=B>> App<(Ia, Ib)> for Snd<A, B>
    where A: TypeOf<Ia>,
          B: TypeOf<Ib>
{
    type Result = Ib;
}


