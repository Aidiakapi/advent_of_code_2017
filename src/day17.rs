framework::day!(17, parse => pt1, pt2);

fn get_slots(step_size: usize, iterations: usize) -> Vec<usize> {
    let mut slots = Vec::with_capacity(iterations);
    slots.push(0);
    for _ in 0..iterations {
        let mut current = slots.len() - 1;
        for _ in 0..step_size {
            current = slots[current];
        }

        let next_index = slots.len();
        let current_slot = &mut slots[current];
        let new_slot = *current_slot;
        *current_slot = next_index;
        slots.push(new_slot);
    }

    slots
}

fn pt1(&step_size: &usize) -> usize {
    let slots = get_slots(step_size, 2017);
    *slots.last().unwrap()
}

fn pt2(&step_size: &usize) -> usize {
    // The chain always starts with [0, ...], because we always insert after the
    // current index.
    //
    // Our answer, is whatever number comes after this, and since nothing can be
    // inserted at index 0, the only insertions that affect this number, are
    // those at index 1.
    //
    // This massively simplifies the problem, since we only have to keep track
    // of the insertion index, and store the latest value we inserted right
    // after zero.

    let mut value_after_zero = 0;
    let mut insertion_index = 0;
    let mut chain_length = 1;
    while chain_length <= 50_000_000 {
        // "Inserts" a single value into the chain, but we only care about it
        // when that value is inserted after 0.
        insertion_index = (insertion_index + step_size) % chain_length;
        if insertion_index == 0 {
            value_after_zero = chain_length;
        }
        insertion_index += 1;
        chain_length += 1;

        // We can greatly reduce the amount of iterations that must be done by
        // advancing many steps at a time. Until we "overrun" the end of the
        // chain again, we will never get back to start.
        let minimal_steps = (chain_length - insertion_index) / (step_size + 1);
        insertion_index += minimal_steps * (step_size + 1);
        chain_length += minimal_steps;
    }

    value_after_zero
}

fn parse(input: &[u8]) -> Result<usize> {
    use parsers::*;
    number::<usize>().execute(input)
}

tests! {
    test_pt!(parse, pt1, b"3" => 638);
}
