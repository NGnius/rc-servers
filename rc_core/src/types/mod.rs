mod account;
pub use account::*;

mod game;
pub use game::*;

pub trait RawType {
    type Raw;

    fn as_raw(&self) -> Self::Raw;

    fn as_raw_ref(&self) -> &'_ Self::Raw;

    fn from_raw(value: Self::Raw) -> Self;
}
