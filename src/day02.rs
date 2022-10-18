use itertools::MinMaxResult;

framework::day!(02, true, parse => pt1, pt2);

fn pt1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|row| match row.iter().minmax() {
            MinMaxResult::NoElements => 0,
            MinMaxResult::OneElement(_) => 0,
            MinMaxResult::MinMax(min, max) => max - min,
        })
        .sum()
}

fn pt2(input: &[Vec<u32>]) -> Result<u32> {
    input
        .iter()
        .map(|row| {
            row.iter()
                .cloned()
                .tuple_combinations()
                .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
                .filter(|(min, max)| max % min == 0)
                .map(|(min, max)| max / min)
                .next()
                .ok_or(Error::InvalidInput("no divisible items"))
        })
        .sum()
}

fn parse(input: &[u8]) -> Result<Vec<Vec<u32>>> {
    use parsers::*;
    number::<u32>()
        .sep_by(token(b'\t').or(token(b' ')))
        .sep_by(token(b'\n'))
        .execute(input)
}

tests! {
    test_pt!(parse, pt1, b"\
5 1 9 5
7 5 3
2 4 6 8" => 18);
    test_pt!(parse, pt2, b"\
5 9 2 8
9 4 7 3
3 8 6 5" => 9);
}
