use crate::day10::full_knot_hash;
use std::fmt::Write;

framework::day!(14, parse => pt1, pt2);

fn iterate_rows(input: &AStr) -> impl Iterator<Item = [u8; 16]> + '_ {
    let mut buffer = AString::with_capacity(input.len() + 4);
    let mut nr_str = String::new();
    buffer.extend_from_slice(input);
    buffer.push(b'-');
    (0..128).map(move |i| {
        _ = write!(nr_str, "{i}");
        buffer.drain(input.len() + 1..);
        buffer.extend_from_slice(nr_str.as_bytes());
        nr_str.clear();
        full_knot_hash(&buffer)
    })
}

fn pt1(input: &AStr) -> u32 {
    iterate_rows(input)
        .map(|hash| hash.into_iter().map(|b| b.count_ones()).sum::<u32>())
        .sum()
}

fn pt2(input: &AStr) -> u32 {
    type V = Vec2<u8>;
    let mut all_cells = HashSet::new();
    for (y, row) in iterate_rows(input).enumerate() {
        let y = y as u8;
        for (byte_index, byte) in row.iter().enumerate() {
            let x = byte_index as u8 * 8;
            for bit in 0..8 {
                if byte & (1 << (7 - bit)) != 0 {
                    all_cells.insert(V::new(x + bit as u8, y));
                }
            }
        }
    }

    let mut regions = 0;
    while !all_cells.is_empty() {
        let starting_point = *all_cells.iter().next().unwrap();
        graph::dfs(starting_point, |point| {
            if all_cells.remove(&point) {
                Some(point.neighbors(&Offset::ORTHOGONAL))
            } else {
                None
            }
            .into_iter()
            .flatten()
        });
        regions += 1;
    }

    regions
}

fn parse(input: &[u8]) -> Result<&AStr> {
    Ok(input.trim_ascii())
}

tests! {
    test_pt!(parse, pt1, b"flqrgnkx" => 8108);
    test_pt!(parse, pt2, b"flqrgnkx" => 1242);
}
