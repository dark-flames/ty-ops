use ty_ops::*;
use ty_ops::classes::*;

#[test]
fn test_list() {
    type L1 = List1<Z>;
    type L2 = Push<S<Z>, L1>;
    type L3 = Push<S<S<Z>>, L2>;

    type Mapped = Map<AddOn<S<Z>>, L3>;

    assert_ty!(List3<S<Z>, S<S<Z>>, S<S<S<Z>>>>, Mapped);
    assert_ty!(Contains<S<S<S<Z>>>, Mapped>, True);
    assert_ty!(Contains<Z, Mapped>, False);
    assert_ty!(Bind<L3, Pure<List<Nat>>>, L3);
}