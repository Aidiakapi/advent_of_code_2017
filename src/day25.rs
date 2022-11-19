use std::collections::hash_map::Entry;

framework::day!(25, parse => pt1, pt2);

struct TuringMachineBlueprint {
    initial_state: u8,
    checksum_after: u32,
    instructions: HashMap<u8, Instruction>,
}

struct TuringMachine<'bp> {
    blueprint: &'bp TuringMachineBlueprint,
    state: u8,
    head: i32,
    tape: HashMap<i32, ()>,
}

impl<'bp> TuringMachine<'bp> {
    fn new(blueprint: &'bp TuringMachineBlueprint) -> TuringMachine {
        TuringMachine {
            blueprint,
            state: blueprint.initial_state,
            head: 0,
            tape: HashMap::new(),
        }
    }

    fn step(&mut self) {
        let instruction = &self.blueprint.instructions[&self.state];

        let current_value = self.tape.entry(self.head);
        let actions = if matches!(current_value, Entry::Occupied(_)) {
            &instruction.when_true
        } else {
            &instruction.when_false
        };

        if actions.write_value {
            current_value.or_default();
        } else if let Entry::Occupied(slot) = current_value {
            slot.remove()
        }
        self.head += match actions.move_direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        self.state = actions.next_state;
    }

    fn checksum(&self) -> usize {
        self.tape.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

struct Instruction {
    when_false: Actions,
    when_true: Actions,
}

struct Actions {
    write_value: bool,
    move_direction: Direction,
    next_state: u8,
}

fn pt1(blueprint: &TuringMachineBlueprint) -> usize {
    let mut tm = TuringMachine::new(blueprint);
    for _ in 0..blueprint.checksum_after {
        tm.step();
    }
    tm.checksum()
}

fn pt2(_: &TuringMachineBlueprint) -> &'static AStr {
    b"gg"
}

fn parse(input: &[u8]) -> Result<TuringMachineBlueprint> {
    use parsers::*;
    let state = pattern!(b'A'..=b'Z');
    let preamble = token(b"Begin in state ")
        .then(state)
        .trailed(token(b".\nPerform a diagnostic checksum after "))
        .and(number::<u32>())
        .trailed(token(b" steps.\n\n"));

    let action = token(b":\n    - Write the value ")
        .then(token((b'0', false)).or(token((b'1', true))))
        .trailed(token(b".\n    - Move one slot to the "))
        .and(token((b"left", Direction::Left)).or(token((b"right", Direction::Right))))
        .trailed(token(b".\n    - Continue with state "))
        .and(state)
        .trailed(token(b"."))
        .map(|((write_value, move_direction), next_state)| Actions {
            write_value,
            move_direction,
            next_state,
        });

    let instruction = token(b"In state ")
        .then(state)
        .trailed(token(b":\n  If the current value is 0"))
        .and(action)
        .trailed(token(b"\n  If the current value is 1"))
        .and(action)
        .map(|((state, when_false), when_true)| {
            (
                state,
                Instruction {
                    when_false,
                    when_true,
                },
            )
        });

    let instructions = instruction.sep_by(token(b"\n\n"));
    preamble
        .and(instructions)
        .map(
            |((initial_state, checksum_after), instructions)| TuringMachineBlueprint {
                initial_state,
                checksum_after,
                instructions,
            },
        )
        .execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

    test_pt!(parse, pt1, EXAMPLE => 3);
}
