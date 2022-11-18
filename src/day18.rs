use std::collections::VecDeque;

framework::day!(18, parse => pt1, pt2);

#[derive(Clone)]
pub struct VM<'i> {
    pub instructions: &'i [Instruction],
    pub registers: [Value; 26],
    pub ip: usize,
}

pub enum ExecRes {
    Continuing,
    Terminated,
    Sending(Register),
    Receiving(Register),
}

impl VM<'_> {
    pub fn new(instructions: &[Instruction]) -> VM {
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

    pub fn next(&mut self) -> ExecRes {
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
                // Only used for day 23:
                Jnz(condition, offset) => {
                    if self.get(condition) != 0 {
                        self.ip = (self.ip as Value + self.get(offset) - 1) as usize;
                    }
                }
                Sub(r, s) => self.registers[*r as usize] -= self.get(s),
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
pub enum Source {
    Register(Register),
    Value(Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Snd(Register),
    Set(Register, Source),
    Add(Register, Source),
    Mul(Register, Source),
    Mod(Register, Source),
    Rcv(Register),
    Jgz(Source, Source),
    // Only used for day 23:
    Jnz(Source, Source),
    Sub(Register, Source),
}

pub fn parse(input: &[u8]) -> Result<Vec<Instruction>> {
    use parsers::*;
    let register = pattern!(b'a'..=b'z').map(|l| Source::Register(l - b'a'));
    let value = number::<Value>().map(Source::Value);
    let source = register.or(value);
    let mnemonic = pattern!(b'a'..=b'z').many_n::<3>();

    #[rustfmt::skip]
    let instruction = mnemonic
        .and(token(b' ').then(source)).and(token(b' ').then(source).opt())
        .map_res(|((mnemonic, a), b)| Ok(match (&mnemonic, a, b) {
            (b"snd", Source::Register(a), None   ) => Instruction::Snd(a   ),
            (b"set", Source::Register(a), Some(b)) => Instruction::Set(a, b),
            (b"add", Source::Register(a), Some(b)) => Instruction::Add(a, b),
            (b"mul", Source::Register(a), Some(b)) => Instruction::Mul(a, b),
            (b"mod", Source::Register(a), Some(b)) => Instruction::Mod(a, b),
            (b"rcv", Source::Register(a), None   ) => Instruction::Rcv(a   ),
            (b"jgz",                  a,  Some(b)) => Instruction::Jgz(a, b),
            // Only used for day 23:
            (b"jnz",                  a,  Some(b)) => Instruction::Jnz(a, b),
            (b"sub", Source::Register(a), Some(b)) => Instruction::Sub(a, b),
            _ => return Err(ParseError::TokenDoesNotMatch),
        }));

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
