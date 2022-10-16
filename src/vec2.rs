#![allow(dead_code)]

use num::{CheckedAdd, CheckedSub, One};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Orthogonal {
    PosX,
    NegX,
    PosY,
    NegY,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Diagonal {
    PosXPosY,
    PosXNegY,
    NegXPosY,
    NegXNegY,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosXPosY,
    PosXNegY,
    NegXPosY,
    NegXNegY,
}

impl<T: Clone + One + CheckedAdd<Output = T> + CheckedSub<Output = T>> Vec2<T> {
    pub fn orthogonal_neighbor(self, direction: Orthogonal) -> Option<Vec2<T>> {
        #[rustfmt::skip]
        match direction {
            Orthogonal::PosX => self.x.checked_add(&num::one()).map(move |x| Vec2 { x, ..self }),
            Orthogonal::NegX => self.x.checked_sub(&num::one()).map(move |x| Vec2 { x, ..self }),
            Orthogonal::PosY => self.y.checked_add(&num::one()).map(move |y| Vec2 { y, ..self }),
            Orthogonal::NegY => self.y.checked_sub(&num::one()).map(move |y| Vec2 { y, ..self }),
        }
    }

    pub fn diagonal_neighbor(self, direction: Diagonal) -> Option<Vec2<T>> {
        #[rustfmt::skip]
        match direction {
            Diagonal::PosXPosY => self.x.checked_add(&num::one()).zip(self.y.checked_add(&num::one())).map(|(x, y)| Vec2 { x, y }),
            Diagonal::PosXNegY => self.x.checked_add(&num::one()).zip(self.y.checked_sub(&num::one())).map(|(x, y)| Vec2 { x, y }),
            Diagonal::NegXPosY => self.x.checked_sub(&num::one()).zip(self.y.checked_add(&num::one())).map(|(x, y)| Vec2 { x, y }),
            Diagonal::NegXNegY => self.x.checked_sub(&num::one()).zip(self.y.checked_sub(&num::one())).map(|(x, y)| Vec2 { x, y }),
        }
    }

    pub fn neighbor(self, direction: Direction) -> Option<Vec2<T>> {
        #[rustfmt::skip]
        match direction {
            Direction::PosX => self.x.checked_add(&num::one()).map(move |x| Vec2 { x, ..self }),
            Direction::NegX => self.x.checked_sub(&num::one()).map(move |x| Vec2 { x, ..self }),
            Direction::PosY => self.y.checked_add(&num::one()).map(move |y| Vec2 { y, ..self }),
            Direction::NegY => self.y.checked_sub(&num::one()).map(move |y| Vec2 { y, ..self }),
            Direction::PosXPosY => self.x.checked_add(&num::one()).zip(self.y.checked_add(&num::one())).map(|(x, y)| Vec2 { x, y }),
            Direction::PosXNegY => self.x.checked_add(&num::one()).zip(self.y.checked_sub(&num::one())).map(|(x, y)| Vec2 { x, y }),
            Direction::NegXPosY => self.x.checked_sub(&num::one()).zip(self.y.checked_add(&num::one())).map(|(x, y)| Vec2 { x, y }),
            Direction::NegXNegY => self.x.checked_sub(&num::one()).zip(self.y.checked_sub(&num::one())).map(|(x, y)| Vec2 { x, y }),
        }
    }

    pub fn orthogonal_neighbors(self) -> impl Iterator<Item = Vec2<T>> {
        Orthogonal::iter().filter_map(move |direction| self.clone().orthogonal_neighbor(direction))
    }
    pub fn diagonal_neighbors(self) -> impl Iterator<Item = Vec2<T>> {
        Diagonal::iter().filter_map(move |direction| self.clone().diagonal_neighbor(direction))
    }
pub     fn neighbors(self) -> impl Iterator<Item = Vec2<T>> {
        Direction::iter().filter_map(move |direction| self.clone().neighbor(direction))
    }
}
