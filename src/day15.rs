framework::day!(15, parse => pt1, pt2);

struct Generator {
    factor: u64,
    value: u64,
}

impl Generator {
    fn next_any(&mut self) -> u64 {
        self.value = self.value * self.factor % 2147483647;
        self.value
    }

    #[inline]
    fn next_filtered(&mut self, multiple_of: u64) -> u64 {
        loop {
            let value = self.next_any();
            if value % multiple_of == 0 {
                break value;
            }
        }
    }
}

fn pt1(&(a, b): &(u64, u64)) -> usize {
    let mut a = Generator {
        factor: 16807,
        value: a,
    };
    let mut b = Generator {
        factor: 48271,
        value: b,
    };

    (0..40_000_000)
        .filter(|_| a.next_any() & 0xffff == b.next_any() & 0xffff)
        .count()
}

fn pt2(&(a, b): &(u64, u64)) -> usize {
    let mut a = Generator {
        factor: 16807,
        value: a,
    };
    let mut b = Generator {
        factor: 48271,
        value: b,
    };

    (0..5_000_000)
        .filter(|_| a.next_filtered(4) & 0xffff == b.next_filtered(8) & 0xffff)
        .count()
}

fn parse(input: &[u8]) -> Result<(u64, u64)> {
    use parsers::*;
    token(b"Generator A starts with ")
        .then(number::<u64>())
        .trailed(token(b"\nGenerator B starts with "))
        .and(number::<u64>())
        .execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
Generator A starts with 65
Generator B starts with 8921
";

    test_pt!(parse, pt1, EXAMPLE => 588);
    test_pt!(parse, pt2, EXAMPLE => 309);
}
