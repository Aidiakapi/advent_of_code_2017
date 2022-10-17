use crate::vec2::Vec2;
use num::{CheckedAdd, CheckedSub, One};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Offset {
    value: u8,
}
impl Offset {
    pub const NONE: Offset = Offset { value: 0b0000 };
    pub const X_POS: Offset = Offset { value: 0b0001 };
    pub const X_NEG: Offset = Offset { value: 0b0010 };
    pub const Y_POS: Offset = Offset { value: 0b0100 };
    pub const Y_NEG: Offset = Offset { value: 0b1000 };
    pub const X_POS_Y_POS: Offset = Offset { value: 0b0101 };
    pub const X_POS_Y_NEG: Offset = Offset { value: 0b1001 };
    pub const X_NEG_Y_POS: Offset = Offset { value: 0b0110 };
    pub const X_NEG_Y_NEG: Offset = Offset { value: 0b1010 };
    pub const ORTHOGONAL: [Offset; 4] =
        [Offset::X_POS, Offset::X_NEG, Offset::Y_POS, Offset::Y_NEG];
    pub const DIAGONAL: [Offset; 4] = [
        Offset::X_POS_Y_POS,
        Offset::X_POS_Y_NEG,
        Offset::X_NEG_Y_POS,
        Offset::X_NEG_Y_NEG,
    ];
    pub const ALL: [Offset; 8] = [
        Offset::X_POS,
        Offset::X_NEG,
        Offset::Y_POS,
        Offset::Y_NEG,
        Offset::X_POS_Y_POS,
        Offset::X_POS_Y_NEG,
        Offset::X_NEG_Y_POS,
        Offset::X_NEG_Y_NEG,
    ];
}

pub trait CompatibleNumber = Clone + CheckedAdd + CheckedSub + One;

pub trait Neighbor: Sized {
    fn neighbor(self, offset: Offset) -> Option<Self>;
}

pub trait Neighbors: Neighbor + Clone {
    fn neighbors<const N: usize>(self, offsets: &'static [Offset; N]) -> NeighborIter<Self, N> {
        NeighborIter {
            base: self,
            offsets,
            index: 0,
        }
    }
}

impl<T: Neighbor + Clone> Neighbors for T {}

pub struct NeighborIter<T: Clone + Neighbor, const N: usize> {
    base: T,
    offsets: &'static [Offset; N],
    index: usize,
}

impl<T: Clone + Neighbor, const N: usize> Iterator for NeighborIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= N {
                return None;
            }
            let value = self.base.clone().neighbor(self.offsets[self.index]);
            self.index += 1;
            if let Some(value) = value {
                return Some(value);
            }
        }
    }
}

impl<T: CompatibleNumber> Neighbor for Vec2<T> {
    fn neighbor(self, offset: Offset) -> Option<Self> {
        let one = T::one();
        let x = match offset.value & 0b0011 {
            0b0001 => self.x.checked_add(&one)?,
            0b0010 => self.x.checked_sub(&one)?,
            _ => self.x,
        };
        let y = match offset.value & 0b1100 {
            0b0100 => self.y.checked_add(&one)?,
            0b1000 => self.y.checked_sub(&one)?,
            _ => self.y,
        };
        Some(Vec2 { x, y })
    }
}
