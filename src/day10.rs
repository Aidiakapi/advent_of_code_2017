framework::day!(10, parse => pt1, pt2);

fn knot_hash<const N: usize, const ROUNDS: usize, T, F: FnOnce(&[u8; N]) -> T>(
    input: &[u8],
    f: F,
) -> T {
    assert!(N <= 256);
    let mut data: [u8; N] = util::init_array(|i| i as u8);

    let mut current_position = 0usize;
    let mut skip_step = 1usize;
    for _ in 0..ROUNDS {
        for length in input.iter().cloned() {
            let mut start = current_position;
            let mut end = current_position
                .wrapping_add(length as usize)
                .wrapping_sub(1);
            current_position = end.wrapping_add(skip_step);
            skip_step += 1;
            while !matches!(start.wrapping_sub(end), 0 | 1) {
                let (s, e) = (start % N, end % N);
                (data[s], data[e]) = (data[e], data[s]);
                start = start.wrapping_add(1);
                end = end.wrapping_sub(1);
            }
        }
    }

    f(&data)
}

pub fn full_knot_hash(input: &[u8]) -> [u8; 16] {
    let lengths: Vec<u8> = input
        .iter()
        .cloned()
        .chain([17, 31, 73, 47, 23])
        .collect();

    knot_hash::<256, 64, _, _>(&lengths, |hash| {
        util::init_array::<u8, 16, _>(|i| {
            let base = i * 16;
            hash[base..base + 16].iter().fold(0, |h, &n| h ^ n)
        })
    })
}

fn pt1_impl<const N: usize>(input: &[u8]) -> Result<MulOutput<[u16; 2]>> {
    use parsers::*;
    let lengths: Vec<_> = number::<u8>().sep_by(token(b',')).execute(input)?;
    Ok(knot_hash::<N, 1, _, _>(&lengths, |hash| {
        MulOutput([hash[0] as u16, hash[1] as u16])
    }))
}

fn pt1(input: &[u8]) -> Result<MulOutput<[u16; 2]>> {
    pt1_impl::<256>(input)
}

fn pt2(input: &[u8]) -> String {
    let hash = full_knot_hash(input);
    use std::fmt::Write;
    hash.into_iter().fold(String::new(), |mut s, v| {
        _ = write!(s, "{v:0>2x}");
        s
    })
}

fn parse(input: &[u8]) -> Result<&[u8]> {
    Ok(input.trim_ascii())
}

tests! {
    test_pt!(parse, pt1, |input| { super::pt1_impl::<5>(&input) },
        b"3,4,1,5" => MulOutput([3, 4])
    );
    test_pt!(parse, pt2,
        b"" => "a2582a3a0e66e6e86e3812dcb672a272",
        b"AoC 2017" => "33efeb34ea91902bb2f59c9920caa6cd",
        b"1,2,3" => "3efbe78a8d82f29979031a4aa0b16a9d",
        b"1,2,4" => "63960835bcdc130f0b66d7ff4f6a5a8e",
    );
}
