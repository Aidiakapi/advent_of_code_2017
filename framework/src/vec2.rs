use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Vec2 { x, y }
    }
}

impl<T: num::Zero> Vec2<T> {
    pub fn zero() -> Self {
        Vec2 {
            x: num::zero(),
            y: num::zero(),
        }
    }
}

pub type Vec2i32 = Vec2<i32>;
pub type Vec2us = Vec2<usize>;

macro_rules! impl_binary_op {
    ($trait:ident, $fn_name:ident, $assign_trait:ident, $assign_fn_name:ident) => {
        impl<T: std::ops::$trait<Rhs, Output = O>, Rhs, O> std::ops::$trait<Vec2<Rhs>> for Vec2<T> {
            type Output = Vec2<O>;

            fn $fn_name(self, rhs: Vec2<Rhs>) -> Self::Output {
                Vec2 {
                    x: self.x.$fn_name(rhs.x),
                    y: self.y.$fn_name(rhs.y),
                }
            }
        }

        impl<T: std::ops::$assign_trait<Rhs>, Rhs> std::ops::$assign_trait<Vec2<Rhs>> for Vec2<T> {
            fn $assign_fn_name(&mut self, rhs: Vec2<Rhs>) {
                self.x.$assign_fn_name(rhs.x);
                self.y.$assign_fn_name(rhs.y);
            }
        }
    };
}

macro_rules! impl_unary_op {
    ($trait:ident, $fn_name:ident) => {
        impl<T: std::ops::$trait<Output = O>, O> std::ops::$trait for Vec2<T> {
            type Output = Vec2<O>;
            fn $fn_name(self) -> Vec2<O> {
                Vec2 {
                    x: self.x.$fn_name(),
                    y: self.y.$fn_name(),
                }
            }
        }
    };
}

impl_binary_op!(Add, add, AddAssign, add_assign);
impl_binary_op!(Sub, sub, SubAssign, sub_assign);
impl_binary_op!(Mul, mul, MulAssign, mul_assign);
impl_binary_op!(Div, div, DivAssign, div_assign);
impl_binary_op!(Rem, rem, RemAssign, rem_assign);
impl_binary_op!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_binary_op!(BitOr, bitor, BitOrAssign, bitor_assign);
impl_binary_op!(BitXor, bitxor, BitXorAssign, bitxor_assign);
impl_binary_op!(Shl, shl, ShlAssign, shl_assign);
impl_binary_op!(Shr, shr, ShrAssign, shr_assign);

impl_unary_op!(Neg, neg);
impl_unary_op!(Not, not);

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
