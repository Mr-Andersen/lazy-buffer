pub trait IdxOpt<I> {
    type Output: ?Sized;

    fn idx_opt<'a>(&'a self, idx: I) -> Option<&Self::Output>;
}

pub trait IdxMutOpt<I> {
    type Output: ?Sized;

    fn idx_mut_opt<'a>(&'a mut self, idx: I) -> Option<&mut Self::Output>;
}

pub trait Len {
    fn len(&self) -> usize;
}

// implementations

use std::collections::VecDeque;

macro_rules! auto_impl {
    ($coll:ident, $( $idx_ty:ty ),* ) => {
$(
impl<T> IdxOpt<$idx_ty> for $coll<T> {
    type Output = T;

    fn idx_opt<'a>(&'a self, idx: $idx_ty) -> Option<&T> {
        self.get(idx)
    }
}
impl<T> IdxMutOpt<$idx_ty> for $coll<T> {
    type Output = T;

    fn idx_mut_opt<'a>(&'a mut self, idx: $idx_ty) -> Option<&mut T> {
        self.get_mut(idx)
    }
}
impl<T> Len for $coll<T> {
    fn len(&self) -> usize {
        self.len()
    }
}
)*
    }
}

auto_impl!(VecDeque, usize);
auto_impl!(Vec, usize);

