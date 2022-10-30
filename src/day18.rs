use std::collections::VecDeque;

framework::day!(18, parse => pt1, pt2);

#[derive(Clone)]
struct VM<'i> {
    instructions: &'i [Instruction],
    registers: [Value; 26],
    ip: usize,
}

enum ExecRes {
    Continuing,
    Terminated,
    Sending(Register),
    Receiving(Register),
}

impl VM<'_> {
    fn new(instructions: &[Instruction]) -> VM {
        VM {
            instructions,
            ip: 0,
            registers: [0; 26],
        }
    }

    fn r(&self, register: u8) -> Value {
        self.registers[register as usize]
    }
    fn get(&self, source: &Source) -> Value {
        match source {
            Source::Register(register) => self.r(*register),
            Source::Value(value) => *value,
        }
    }

    fn next(&mut self) -> ExecRes {
        if let Some(instruction) = self.instructions.get(self.ip) {
            self.ip += 1;

            use Instruction::*;
            match instruction {
                Snd(r) => {
                    return ExecRes::Sending(*r);
                }
                Set(r, s) => self.registers[*r as usize] = self.get(s),
                Add(r, s) => self.registers[*r as usize] += self.get(s),
                Mul(r, s) => self.registers[*r as usize] *= self.get(s),
                Mod(r, s) => self.registers[*r as usize] %= self.get(s),
                Rcv(r) => {
                    return ExecRes::Receiving(*r);
                }
                Jgz(condition, offset) => {
                    if self.get(condition) > 0 {
                        self.ip = (self.ip as Value + self.get(offset) - 1) as usize;
                    }
                }
            }
            ExecRes::Continuing
        } else {
            ExecRes::Terminated
        }
    }
}

fn pt1(instructions: &[Instruction]) -> Result<Value> {
    let mut vm = VM::new(instructions);
    let mut last_value = None;
    loop {
        match vm.next() {
            ExecRes::Continuing => {}
            ExecRes::Terminated => {
                break Err(Error::InvalidInput("exited before a value was recalled"))
            }
            ExecRes::Sending(r) => last_value = Some(vm.r(r)),
            ExecRes::Receiving(r) => {
                if vm.r(r) != 0 {
                    break last_value
                        .ok_or(Error::InvalidInput("Recover before any sound was played"));
                }
            }
        }
    }
}

fn pt2(instructions: &[Instruction]) -> usize {
    #[derive(Clone)]
    struct State<'i> {
        vm: VM<'i>,
        message_queue: VecDeque<Value>,
        sent_count: Option<usize>,
    }
    let mut active = State {
        vm: VM::new(instructions),
        message_queue: VecDeque::new(),
        sent_count: None,
    };
    let mut passive = active.clone();
    passive.vm.registers[(b'p' - b'a') as usize] = 1;
    passive.sent_count = Some(0usize);

    let mut just_swapped = false;
    loop {
        let can_continue = match active.vm.next() {
            ExecRes::Continuing => true,
            ExecRes::Terminated => false,
            ExecRes::Sending(r) => {
                passive.message_queue.push_back(active.vm.r(r));
                if let Some(sent_count) = &mut active.sent_count {
                    *sent_count += 1;
                }
                true
            }
            ExecRes::Receiving(r) => {
                if let Some(value) = active.message_queue.pop_front() {
                    active.vm.registers[r as usize] = value;
                    true
                } else {
                    active.vm.ip -= 1;
                    false
                }
            }
        };
        if can_continue {
            just_swapped = false;
            continue;
        }
        if just_swapped {
            break;
        }
        just_swapped = true;
        std::mem::swap(&mut active, &mut passive);
    }
    active.sent_count.or(passive.sent_count).unwrap()
}

type Register = u8;
type Value = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Source {
    Register(Register),
    Value(Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Snd(Register),
    Set(Register, Source),
    Add(Register, Source),
    Mul(Register, Source),
    Mod(Register, Source),
    Rcv(Register),
    Jgz(Source, Source),
}

fn parse(input: &[u8]) -> Result<Vec<Instruction>> {
    use parsers::*;
    let register = pattern!(b'a'..=b'z').map(|l| l - b'a');
    let value = number::<Value>();
    let source = register
        .map(|r| Source::Register(r))
        .or(value.map(|v| Source::Value(v)));
    let register_source = register.trailed(token(b' ')).and(source);
    let source_source = source.trailed(token(b' ')).and(source);
    #[rustfmt::skip]
    let instruction =
            token(b"snd ").then(register)       .map(| r    | Instruction::Snd(r))
        .or(token(b"set ").then(register_source).map(|(r, s)| Instruction::Set(r, s)))
        .or(token(b"add ").then(register_source).map(|(r, s)| Instruction::Add(r, s)))
        .or(token(b"mul ").then(register_source).map(|(r, s)| Instruction::Mul(r, s)))
        .or(token(b"mod ").then(register_source).map(|(r, s)| Instruction::Mod(r, s)))
        .or(token(b"rcv ").then(register)       .map(| r    | Instruction::Rcv(r)))
        .or(token(b"jgz ").then(  source_source).map(|(a, b)| Instruction::Jgz(a, b)));

    instruction.sep_by(token(b'\n')).execute(input)
}

tests! {
    test_pt!(parse, pt1, b"\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
" => 4);
}
