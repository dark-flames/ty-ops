pub trait Type: 'static + Clone + Copy + Default {}

pub trait TypeOf<T>: Type {}

pub trait Value {
    type Type: Type;
}

#[macro_export]
macro_rules! assert_ty {
    ($a: ty, $b: ty) => {
        let _: $a = <$b>::default();
    }
}

