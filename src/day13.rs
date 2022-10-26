use num::Integer;
use std::mem::swap;

framework::day!(13, parse => pt1, pt2);

#[derive(Debug, Clone)]
struct Layer {
    index: u32,
    depth: u32,
}

impl Layer {
    fn interval(&self) -> u32 {
        self.depth * 2 - 2
    }
}

fn pt1(layers: &[Layer]) -> u32 {
    layers
        .iter()
        .map(|layer| {
            if layer.index % layer.interval() == 0 {
                layer.index * layer.depth
            } else {
                0
            }
        })
        .sum()
}

fn pt2(layers: &[Layer]) -> u32 {
    // We represent the infinite sequence of starting time for which no
    // collisions, via:
    // - A base, which can be multiplier with any number 0 through infinity
    // - Offsets, which are added to the base
    //
    // For example, with base = 4, and offsets = [1, 2], we represent the
    // infinite sequence:
    // [1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, ...]
    //
    // We start out at base = 1, and offsets = [0], which represents all
    // positive integers, starting at 0.
    //
    // For each layer we pass through, we expand the base to be a multiple of
    // the interval at which this layer repeats. This is done by repeating any
    // offsets that now no longer fall into the base. Let's say we have a layer:
    // interval = 10, index = 1
    //
    // The least common multiple between 4 and 10, is 20, so:
    // base changes from 4 => 20
    // offsets changes from [1, 2] => [1, 2, 5, 6, 9, 10, 13, 14, 17, 18]
    //
    // Since our base is now a multiple of the interval, any non-colliding
    // offsets, will never cause a collision.
    // This means we simply have to filter the offsets, based on whether they
    // collide. It collides when: (offset + index) % interval == 0.
    // In our sequence, the only colliding number is removed: (9 + 1) % 10 == 0
    // Base: 20
    // Result: [1, 2, 5, 6, 10, 13, 14, 17, 18]
    //
    // This process is repeated for each layer.
    //
    // The end result is a representation of the infinite sequence of all
    // starting times, which result in no collisions. The answer is simply the
    // first number in that sequence.

    let mut base = 1;
    let mut offsets = vec![0];
    let mut temp = vec![];
    for layer in layers {
        let (index, interval) = (layer.index, layer.interval());

        let new_base = base;
        base = new_base.lcm(&interval);

        for i in 0..base / new_base {
            let m = i * new_base;
            temp.extend(
                offsets
                    .iter()
                    .map(|&n| n + m)
                    .filter(|n| (n + index) % interval != 0),
            );
        }
        swap(&mut offsets, &mut temp);
        temp.clear();
    }
    offsets[0]
}

fn parse(input: &[u8]) -> Result<Vec<Layer>> {
    use parsers::*;
    let layer = number::<u32>().trailed(token(b": ")).and(number::<u32>());
    let layer = layer.map(|(index, depth)| Layer { index, depth });
    layer.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
0: 3
1: 2
4: 4
6: 4";

    test_pt!(parse, pt1, EXAMPLE => 24);
    test_pt!(parse, pt2, EXAMPLE => 10);
}
