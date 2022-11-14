#![allow(clippy::unusual_byte_groupings)]

use bitvec::prelude::*;
use std::ops::Index;

framework::day!(21, parse => pt1, pt2);

type Vec2 = framework::vecs::Vec2<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Grid<const N: usize>(u16);

trait Transformable: Copy + Ord {
    fn flip_v(self) -> Self;
    fn rot_90(self) -> Self;
    fn normalize(self) -> Self {
        [self, self.flip_v()]
            .into_iter()
            .flat_map(|v| {
                let r90 = v.rot_90();
                let r180 = r90.rot_90();
                let r270 = r180.rot_90();
                [v, r90, r180, r270]
            })
            .min()
            .unwrap()
    }
}

#[rustfmt::skip]
impl Transformable for Grid<2> {
    fn flip_v(self) -> Self {
        Self(
            (self.0 & 0b_00_11) << 2 |
            (self.0 & 0b_11_00) >> 2
        )
    }

    fn rot_90(self) -> Self {
        Self(
            (self.0 & 0b_00_01) << 1 |
            (self.0 & 0b_00_10) << 2 |
            (self.0 & 0b_01_00) >> 2 |
            (self.0 & 0b_10_00) >> 1
        )
    }
}

#[rustfmt::skip]
impl Transformable for Grid<3> {
    fn flip_v(self) -> Self {
        Self(
            (self.0 & 0b_000_000_111) << 6 |
            (self.0 & 0b_000_111_000)      |
            (self.0 & 0b_111_000_000) >> 6
        )
    }

    fn rot_90(self) -> Self {
        Self(
            (self.0 & 0b_000_100_001) << 2 |
            (self.0 & 0b_000_000_010) << 4 |
            (self.0 & 0b_000_000_100) << 6 |
            (self.0 & 0b_100_001_000) >> 2 |
            (self.0 & 0b_000_010_000)      |
            (self.0 & 0b_001_000_000) >> 6 |
            (self.0 & 0b_010_000_000) >> 4
        )
    }
}

impl Index<Vec2> for Grid<3> {
    type Output = bool;

    fn index(&self, index: Vec2) -> &bool {
        debug_assert!(index.x < 3);
        debug_assert!(index.y < 3);
        let bit_index = index.x + index.y * 3;
        if (self.0 >> bit_index) & 1 == 1 {
            &true
        } else {
            &false
        }
    }
}

impl Index<Vec2> for Grid<4> {
    type Output = bool;

    fn index(&self, index: Vec2) -> &bool {
        debug_assert!(index.x < 4);
        debug_assert!(index.y < 4);
        let bit_index = index.x + index.y * 4;
        if (self.0 >> bit_index) & 1 == 1 {
            &true
        } else {
            &false
        }
    }
}

struct Rules {
    two_to_three: Vec<(Grid<2>, Grid<3>)>,
    three_to_four: Vec<(Grid<3>, Grid<4>)>,
}

const fn count_size_after_iterations<const N: usize>() -> usize {
    let mut s = 3;
    let mut i = 0;
    while i < N {
        s += if s % 2 == 0 { s / 2 } else { s / 3 };
        i += 1;
    }
    s
}

struct Board<const N: usize> {
    data: BitVec<usize, LocalBits>,
}

impl<const N: usize> Board<N> {
    const SIZE: usize = count_size_after_iterations::<N>();
    pub fn new() -> Self {
        Self {
            data: BitVec::repeat(false, Self::SIZE * Self::SIZE),
        }
    }

    pub fn set(&mut self, index: Vec2, value: bool) {
        debug_assert!(index.x < Self::SIZE);
        debug_assert!(index.y < Self::SIZE);
        self.data.set(index.x + index.y * Self::SIZE, value);
    }
}

impl<const N: usize> Index<Vec2> for Board<N> {
    type Output = bool;

    fn index(&self, index: Vec2) -> &Self::Output {
        debug_assert!(index.x < Self::SIZE);
        debug_assert!(index.y < Self::SIZE);
        &self.data[index.x + index.y * Self::SIZE]
    }
}

fn apply_transformation<const B: usize, const N: usize>(
    board: &mut Board<B>,
    size: &mut usize,
    lookup: &HashMap<Grid<N>, Grid<{ N + 1 }>>,
) where
    Grid<N>: Transformable,
    Grid<{ N + 1 }>: Index<Vec2, Output = bool>,
{
    let block_size = N;
    let block_count = *size / block_size;
    let new_block_size = N + 1;

    for by in (0..block_count).rev() {
        for bx in (0..block_count).rev() {
            let src = Vec2::new(bx * block_size, by * block_size);
            let dst = Vec2::new(bx * new_block_size, by * new_block_size);
            let mut data = 0;
            for y in 0..N {
                for x in 0..N {
                    let offset = Vec2::new(x, y);
                    data = data << 1 | (board[src + offset] as u16);
                }
            }
            let data = Grid::<N>(data).normalize();
            let new_data = lookup[&data];
            for y in (0..N + 1).rev() {
                for x in (0..N + 1).rev() {
                    let offset = Vec2::new(x, y);
                    board.set(dst + offset, new_data[offset]);
                }
            }
        }
    }

    *size += block_count;
}

fn count_after_iterations<const N: usize>(rules: &Rules) -> usize {
    let mut board = Board::<N>::new();
    board.set((1, 0).into(), true);
    board.set((2, 1).into(), true);
    board.set((0, 2).into(), true);
    board.set((1, 2).into(), true);
    board.set((2, 2).into(), true);

    let two_to_three = rules
        .two_to_three
        .iter()
        .map(|&(from, to)| (from.normalize(), to))
        .collect::<HashMap<_, _>>();
    let three_to_four = rules
        .three_to_four
        .iter()
        .map(|&(from, to)| (from.normalize(), to))
        .collect::<HashMap<_, _>>();

    let mut size = 3;

    for _ in 0..N {
        if size % 2 == 0 {
            apply_transformation(&mut board, &mut size, &two_to_three);
        } else {
            apply_transformation(&mut board, &mut size, &three_to_four);
        }
    }

    board.data.count_ones()
}

fn pt1(rules: &Rules) -> usize {
    count_after_iterations::<5>(rules)
}

fn pt2(rules: &Rules) -> usize {
    count_after_iterations::<18>(rules)
}

fn parse(input: &[u8]) -> Result<Rules> {
    use parsers::*;

    let bit = token((b'.', false)).or(token((b'#', true)));
    let sep = token(b'/');
    let arrow = token(b" => ");

    let b2 = bit.many_n::<2>().map(|[a, b]| a as u16 | (b as u16) << 1);
    let b3 = bit
        .many_n::<3>()
        .map(|[a, b, c]| a as u16 | (b as u16) << 1 | (c as u16) << 2);
    let b4 = bit
        .many_n::<4>()
        .map(|[a, b, c, d]| a as u16 | (b as u16) << 1 | (c as u16) << 2 | (d as u16) << 3);
    let b2x2 = b2.and(sep.then(b2)).map(|(r1, r2)| Grid::<2>(r1 | r2 << 2));
    let b3x3 = b3
        .and(sep.then(b3))
        .and(sep.then(b3))
        .map(|((r1, r2), r3)| Grid::<3>(r1 | r2 << 3 | r3 << 6));
    let b4x4 = b4
        .and(sep.then(b4))
        .and(sep.then(b4))
        .and(sep.then(b4))
        .map(|(((r1, r2), r3), r4)| Grid::<4>(r1 | r2 << 4 | r3 << 8 | r4 << 12));

    let two_to_three = b2x2.and(arrow.then(b3x3)).sep_by(token(b'\n'));
    let three_to_four = b3x3.and(arrow.then(b4x4)).sep_by(token(b'\n'));

    two_to_three
        .and(token(b'\n').then(three_to_four))
        .map(|(two_to_three, three_to_four)| Rules {
            two_to_three,
            three_to_four,
        })
        .execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    #[test]
    fn transformations() {
        for case in [
            (0b_00_01, 0b_01_00, 0b_00_10),
            (0b_00_10, 0b_10_00, 0b_10_00),
            (0b_01_00, 0b_00_01, 0b_00_01),
            (0b_10_00, 0b_00_10, 0b_01_00),
        ]
        {
            assert_eq!(Grid::<2>(case.0).flip_v(), Grid::<2>(case.1));
            assert_eq!(Grid::<2>(case.0).rot_90(), Grid::<2>(case.2));
        }
        for case in [
            (0b_000_000_001, 0b_001_000_000, 0b_000_000_100),
            (0b_000_000_010, 0b_010_000_000, 0b_000_100_000),
            (0b_000_000_100, 0b_100_000_000, 0b_100_000_000),
            (0b_000_001_000, 0b_000_001_000, 0b_000_000_010),
            (0b_000_010_000, 0b_000_010_000, 0b_000_010_000),
            (0b_000_100_000, 0b_000_100_000, 0b_010_000_000),
            (0b_001_000_000, 0b_000_000_001, 0b_000_000_001),
            (0b_010_000_000, 0b_000_000_010, 0b_000_001_000),
            (0b_100_000_000, 0b_000_000_100, 0b_001_000_000),
        ]
        {
            assert_eq!(Grid::<3>(case.0).flip_v(), Grid::<3>(case.1));
            assert_eq!(Grid::<3>(case.0).rot_90(), Grid::<3>(case.2));
        }
    }

    test_pt!(parse, pt1, |input| { super::count_after_iterations::<2>(&input) },
        EXAMPLE => 12);
}
