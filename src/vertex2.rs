//! Represents a 2D point in space.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vertex2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vertex2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

macro_rules! bin_op_impl {
    ($trait:ident, $method:ident, $op:tt, $lhs:ty, $rhs:ty) => {
        impl<T> $trait<$rhs> for $lhs
        where
            T: $trait<Output = T> + Copy,
        {
            type Output = Vertex2<T>;

            fn $method(self, rhs: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                }
            }
        }
    };
}

macro_rules! bin_op_with_ref_impl {
    ($trait:ident, $method:ident, $op:tt, $ty:ty) => {
        bin_op_impl!($trait, $method, $op, $ty, $ty);
        bin_op_impl!($trait, $method, $op, $ty, &$ty);
        bin_op_impl!($trait, $method, $op, $ty, &mut $ty);

        bin_op_impl!($trait, $method, $op, &$ty, $ty);
        bin_op_impl!($trait, $method, $op, &$ty, &$ty);
        bin_op_impl!($trait, $method, $op, &$ty, &mut $ty);

        bin_op_impl!($trait, $method, $op, &mut $ty, $ty);
        bin_op_impl!($trait, $method, $op, &mut $ty, &$ty);
        bin_op_impl!($trait, $method, $op, &mut $ty, &mut $ty);
    };
}

bin_op_with_ref_impl!(Add, add, +, Vertex2<T>);
bin_op_with_ref_impl!(Sub, sub, -, Vertex2<T>);
bin_op_with_ref_impl!(Mul, mul, *, Vertex2<T>);
bin_op_with_ref_impl!(Div, div, /, Vertex2<T>);

macro_rules! bin_op_assign_impl {
    ($trait:ident, $method:ident, $op:tt, $lhs:ty, $rhs:ty) => {
        impl<T> $trait<$rhs> for $lhs
        where
            T: $trait + Copy,
        {
            fn $method(&mut self, rhs: $rhs) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }
    };
}

macro_rules! bin_op_assign_with_ref_impl {
    ($trait:ident, $method:ident, $op:tt, $ty:ty) => {
        bin_op_assign_impl!($trait, $method, $op, $ty, $ty);
        bin_op_assign_impl!($trait, $method, $op, $ty, &$ty);
        bin_op_assign_impl!($trait, $method, $op, $ty, &mut $ty);

        bin_op_assign_impl!($trait, $method, $op, &mut $ty, $ty);
        bin_op_assign_impl!($trait, $method, $op, &mut $ty, &$ty);
        bin_op_assign_impl!($trait, $method, $op, &mut $ty, &mut $ty);
    };
}

bin_op_assign_with_ref_impl!(AddAssign, add_assign, +=, Vertex2<T>);
bin_op_assign_with_ref_impl!(SubAssign, sub_assign, -=, Vertex2<T>);
bin_op_assign_with_ref_impl!(MulAssign, mul_assign, *=, Vertex2<T>);
bin_op_assign_with_ref_impl!(DivAssign, div_assign, /=, Vertex2<T>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let vertex = Vertex2::new(1.0, 2.0);
        assert_eq!(vertex.x, 1.0);
        assert_eq!(vertex.y, 2.0);
    }

    #[test]
    fn add() {
        let vertex = Vertex2::new(1.5, 2.0) + Vertex2::new(2.0, 3.0);
        assert_eq!(vertex.x, 3.5);
        assert_eq!(vertex.y, 5.0);
    }

    #[test]
    fn sub() {
        let vertex = Vertex2::new(1.5, 2.0) - Vertex2::new(2.0, 3.0);
        assert_eq!(vertex.x, -0.5);
        assert_eq!(vertex.y, -1.0);
    }

    #[test]
    fn mul() {
        let vertex = Vertex2::new(1.5, 2.0) * Vertex2::new(2.0, 3.0);
        assert_eq!(vertex.x, 3.0);
        assert_eq!(vertex.y, 6.0);
    }

    #[test]
    fn div() {
        let vertex = Vertex2::new(1.0, 2.0) / Vertex2::new(2.0, 4.0);
        assert_eq!(vertex.x, 0.5);
        assert_eq!(vertex.y, 0.5);
    }

    #[test]
    fn add_assign() {
        let mut vertex = Vertex2::new(1.0, 2.0);
        vertex += Vertex2::new(3.0, 4.0);
        assert_eq!(vertex.x, 4.0);
        assert_eq!(vertex.y, 6.0);
    }

    #[test]
    fn sub_assign() {
        let mut vertex = Vertex2::new(1.0, 2.0);
        vertex -= Vertex2::new(3.0, 1.0);
        assert_eq!(vertex.x, -2.0);
        assert_eq!(vertex.y, 1.0);
    }

    #[test]
    fn mul_assign() {
        let mut vertex = Vertex2::new(2.0, 4.0);
        vertex *= Vertex2::new(3.0, 3.0);
        assert_eq!(vertex.x, 6.0);
        assert_eq!(vertex.y, 12.0);
    }

    #[test]
    fn div_assign() {
        let mut vertex = Vertex2::new(2.0, 10.0);
        vertex /= Vertex2::new(4.0, 5.0);
        assert_eq!(vertex.x, 0.5);
        assert_eq!(vertex.y, 2.0);
    }
}
