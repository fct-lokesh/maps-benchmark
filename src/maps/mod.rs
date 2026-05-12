pub mod hm;
pub mod lmv1;
pub mod lmv2;
pub mod lmv3;

pub trait Finder {
    type KeyType;

    type ValType<'a>;

    fn new() -> Self;

    fn push<'a>(&mut self, key: Self::KeyType, val: Self::ValType<'a>);

    fn find<'a>(&self, data: Self::KeyType) -> Option<Self::ValType<'a>>;
}
