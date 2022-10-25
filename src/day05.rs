framework::day!(05, parse => pt1, pt2);

fn pts<F: Fn(i32) -> i32>(input: &[i32], remap: F) -> usize {
    let mut input = input.to_vec();
    let mut counter = 0;
    let mut ip = 0i32;
    loop {
        match ip.try_into().ok().and_then(|idx: usize| input.get_mut(idx)) {
            Some(v) => {
                counter += 1;
                ip += *v;
                *v = remap(*v);
            }
            None => break counter,
        }
    }
}

fn pt1(input: &[i32]) -> usize {
    pts(input, |n| n + 1)
}

fn pt2(input: &[i32]) -> usize {
    pts(input, |n| if n >= 3 { n - 1 } else { n + 1 })
}

fn parse(input: &[u8]) -> Result<Vec<i32>> {
    use parsers::*;
    number::<i32>().sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
0
3
0
1
-3";

    test_pt!(parse, pt1, EXAMPLE => 5);
    test_pt!(parse, pt2, EXAMPLE => 10);
}
