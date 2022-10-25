use std::str::FromStr;
use strum_macros::EnumString;

framework::day!(08, parse => pt1, pt2);

fn execute_instructions<F: FnMut(i32)>(instructions: &[Instruction], mut f: F) -> HashMap<Register, i32> {
    let mut registers = HashMap::<Register, i32>::new();
    for instruction in instructions {
        let (reg, condition, threshold) = &instruction.condition;
        let a = registers.get(reg).cloned().unwrap_or(0);
        let b = *threshold;
        let matches = match condition {
            Condition::Equal => a == b,
            Condition::NotEqual => a != b,
            Condition::GreaterOrEqual => a >= b,
            Condition::LesserOrEqual => a <= b,
            Condition::Greater => a > b,
            Condition::Lesser => a < b,
        };
        if matches {
            let value = registers.entry(instruction.target.clone()).or_insert(0);
            *value += instruction.offset;
            f(*value);
        }
    }
    registers
}

fn pt1(instructions: &[Instruction]) -> i32 {
    let registers = execute_instructions(instructions, |_| {});
    registers.values().cloned().max().unwrap()
}

fn pt2(instructions: &[Instruction]) -> i32 {
    let mut max = 0i32;
    execute_instructions(instructions, |v| max = max.max(v));
    max
}

type Register = ArrayVec<u8, 4>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    target: Register,
    offset: i32,
    condition: (Register, Condition, i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString)]
enum Condition {
    #[strum(serialize = "==")]
    Equal,
    #[strum(serialize = "!=")]
    NotEqual,
    #[strum(serialize = ">=")]
    GreaterOrEqual,
    #[strum(serialize = "<=")]
    LesserOrEqual,
    #[strum(serialize = ">")]
    Greater,
    #[strum(serialize = "<")]
    Lesser,
}

fn parse(input: &[u8]) -> Result<Vec<Instruction>> {
    use parsers::*;
    let register = pattern!(b'a'..=b'z').repeat_into::<Register>();
    let inc = token(b" inc ").then(number::<i32>());
    let dec = token(b" dec ").then(number::<i32>()).map(|n| -n);
    let offset = inc.or(dec);
    let condition = token(b" if ")
        .then(register.clone())
        .trailed(token(b' '))
        .and(
            pattern!(b'=' | b'>' | b'<' | b'!')
                .fold_mut(ArrayString::<2>::new(), |v, l| v.push(l as char))
                .map_res(|v| {
                    Condition::from_str(v.as_str())
                        .map_err(|_| ParseError::Custom("expected condition"))
                }),
        )
        .trailed(token(b' '))
        .and(number::<i32>())
        .map(|((register, condition), value)| (register, condition, value));

    let instruction = register
        .and(offset)
        .and(condition)
        .map(|((target, offset), condition)| Instruction {
            target,
            offset,
            condition,
        });

    instruction.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    test_pt!(parse, pt1, EXAMPLE => 1);
    test_pt!(parse, pt2, EXAMPLE => 10);
}
