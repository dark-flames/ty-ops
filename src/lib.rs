#![feature(associated_type_bounds)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

mod nat;
mod lambda;
mod product;
mod ty;
mod boolean;
mod list;
mod eq;
mod maybe;
mod monad;
mod functor;

pub use nat::*;
pub use lambda::*;
pub use product::*;
pub use ty::*;
pub use boolean::*;
pub use eq::*;
pub use list::*;
pub use maybe::*;
pub use monad::*;
pub use functor::*;