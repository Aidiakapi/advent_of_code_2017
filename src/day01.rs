framework::day!(01, parse => pt1, pt2);

fn sum_if_identical_when_offset(input: &[u8], offset: usize) -> u32 {
    input
        .iter()
        .zip(input.iter().cycle().skip(offset))
        .filter(|(&a, &b)| a == b)
        .map(|(&x, _)| x as u32)
        .sum()
}

fn pt1(input: &[u8]) -> u32 {
    sum_if_identical_when_offset(input, 1)
}

fn pt2(input: &[u8]) -> u32 {
    sum_if_identical_when_offset(input, input.len() / 2)
}

fn parse(input: &[u8]) -> Result<Vec<u8>> {
    use parsers::*;
    digit().repeat_into().execute(input)
}

tests! {
    test_pt!(parse, pt1,
        b"1122" => 3,
        b"1111" => 4,
        b"1234" => 0,
        b"91212129" => 9,
    );
    test_pt!(parse, pt2,
        b"1212" => 6,
        b"1221" => 0,
        b"123425" => 4,
        b"123123" => 12,
        b"12131415" => 4,
    );
}
