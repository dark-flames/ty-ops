use std::marker::PhantomData;

use crate::*;

pub type List1<I> = Segment<I, Empty<<I as Value>::Type>>;
pub type List2<I1, I2> = Push<I2, List1<I1>>;
pub type List3<I1, I2, I3> = Push<I3, List2<I1, I2>>;

#[derive(Clone, Copy, Default)]
pub struct List<T: Type>(PhantomData<T>);

impl<T: Type> Type for List<T> {}

#[derive(Clone, Copy, Default)]
pub struct Empty<T: Type>(PhantomData<T>);

#[derive(Clone, Copy, Default)]
pub struct Segment<I: Value, Next: Value<Type=List<<I as Value>::Type>>>(PhantomData<(I, Next)>);

impl<T: Type> Value for Empty<T> {
    type Type = List<T>;
}

impl<T: Type, I: Value<Type=T>, Next: Value<Type=List<T>>> Value for Segment<I, Next> {
    type Type = List<T>;
}

pub type Push<I, L> = Eval<PushTo<I>, L>;
#[derive(Clone, Copy, Default)]
pub struct PushTo<I: Value>(PhantomData<I>);

impl<T: Type, I: Value<Type=T>> Value for PushTo<I> {
    type Type = Lambda<List<T>, List<T>>;
}

impl<T: Type, I: Value<Type=T>, L: Value<Type=List<T>>>  App<L> for PushTo<I> {
    type Result = Segment<I, L>;
}

pub type Map<F, L> = Eval<MapOn<F>, L>;
#[derive(Clone, Copy, Default)]
pub struct MapOn<F: Value>(PhantomData<F>);

impl<A: Type, B: Type, F: Value<Type=Lambda<A, B>>> Value for MapOn<F> {
    type Type = Lambda<List<A>, List<B>>;
}

impl<A: Type, B: Type, F: Value<Type=Lambda<A, B>>> App<Empty<A>> for MapOn<F> {
    type Result = Empty<B>;
}

impl<
    A: Type, B: Type,
    F: Value<Type=Lambda<A, B>> + App<Ia, Result=Ib>,
    Ia: Value<Type=A>,
    Ib: Value<Type=B>,
    La: Value<Type=List<A>>,
    Lb: Value<Type=List<B>>,
> App<Segment<Ia, La>> for MapOn<F>
    where
        MapOn<F>: Value<Type=Lambda<List<A>, List<B>>> + App<La, Result=Lb>
{
    type Result = Segment<
        Ib,
        Lb
    >;
}

// count
pub type Contains<I, L> = Eval<ContainIn<I>, L>;
pub struct ContainIn<I: Value>(PhantomData<I>);

impl<T: Type, I: Value<Type=T>> Value for ContainIn<I> {
    type Type = Lambda<List<T>, Bool>;
}

impl<T: Type, I: Value<Type=T>> App<Empty<T>> for ContainIn<I> {
    type Result = False;
}

impl<
    T: Type,
    I: Value<Type=T>,
    LI: Value<Type=T>,
    L: Value<Type=List<T>>,
    Eq: Value<Type=Bool>,
    C: Value<Type=Bool>,
    R: Value<Type=Bool>
> App<Segment<LI, L>> for ContainIn<I>
    where
        EqTo<I>: App<LI, Result=Eq>,
        ContainIn<I>: App<L, Result=C>,
        OrOn<Eq>: App<C, Result=R>
{
    type Result = R;
}

#[test]
fn test_list() {
    use crate::*;
    type L1 = List1<Z>;
    type L2 = Push<S<Z>, L1>;
    type L3 = Push<S<S<Z>>, L2>;

    type Mapped = Map<AddOn<S<Z>>, L3>;

    assert_ty!(List3<S<Z>, S<S<Z>>, S<S<S<Z>>>>, Mapped);
    assert_ty!(Contains<S<S<S<Z>>>, Mapped>, True);
    assert_ty!(Contains<Z, Mapped>, False);
}

