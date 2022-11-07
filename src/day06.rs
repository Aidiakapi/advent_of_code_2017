framework::day!(06, parse => pt1, pt2);

fn pts<F: FnMut(&[u32]) -> bool>(input: &[u32], mut should_stop: F) {
    let mut memory_banks = input.to_vec();
    loop {
        if should_stop(&memory_banks) {
            break;
        }
        let highest_index =
            memory_banks.len() - 1 - memory_banks.iter().cloned().rev().position_max().unwrap();
        let blocks = memory_banks[highest_index];
        memory_banks[highest_index] = 0;
        for idx in (0..memory_banks.len())
            .cycle()
            .skip(highest_index + 1)
            .take(blocks as usize)
        {
            memory_banks[idx] += 1;
        }
    }
}

fn pt1(input: &[u32]) -> usize {
    let mut seen_banks = HashSet::new();
    let mut iteration = 0;
    pts(input, |memory_banks| {
        if !seen_banks.insert(memory_banks.to_vec()) {
            return true;
        }
        iteration += 1;
        false
    });
    iteration
}

fn pt2(input: &[u32]) -> usize {
    let mut seen_banks = HashMap::new();
    let mut iteration = 0;
    pts(input, |memory_banks| {
        if let Some(prev_iteration) = seen_banks.insert(memory_banks.to_vec(), iteration) {
            iteration -= prev_iteration;
            return true;
        }
        iteration += 1;
        false
    });
    iteration
}

fn parse(input: &[u8]) -> Result<Vec<u32>> {
    use parsers::*;
    number::<u32>().sep_by(token(b'\t')).execute(input)
}

tests! {
    test_pt!(parse, pt1, b"0\t2\t7\t0" => 5);
    test_pt!(parse, pt2, b"0\t2\t7\t0" => 4);
}
