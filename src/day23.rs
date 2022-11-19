use num::integer::Roots;

use crate::day18::{self, ExecRes, Instruction, Source, VM};

framework::day!(23, parse => pt1, pt2);

fn pt1(instructions: &[Instruction]) -> Result<u32> {
    let mut vm = VM::new(instructions);
    let mut mul_count = 0;
    loop {
        if let Some(Instruction::Mul(_, _)) = vm.instructions.get(vm.ip) {
            mul_count += 1;
        }
        match vm.next() {
            ExecRes::Continuing => continue,
            ExecRes::Terminated => break,
            _ => {
                return Err(Error::InvalidInput(
                    "transmission instructions not supported",
                ))
            }
        }
    }
    Ok(mul_count)
}

// See day23_analysis.txt for details for the process in which I analyzed the
// input, and determined the algorithm.
//
// The distinction between different inputs, is the initial value of the `b`
// register, which ends up setting a lower and upper bound on the iteration.
//
// It iterates between the bounds (inclusive), and counts the amount of numbers
// which are not prime.
fn pt2(instructions: &[Instruction]) -> Result<usize> {
    let input: u64 = if let Instruction::Set(1, Source::Value(input)) = instructions[0] {
        input.try_into().ok().ok_or(Error::InvalidInput(
            "initial value for b cannot be negative",
        ))?
    } else {
        return Err(Error::InvalidInput("expected input to start with: set b X"));
    };

    let lower = input * 100 + 100_000;
    let upper = lower + 17_000;
    const STEP_BY: usize = 17;

    let mut primes = Primes::new();
    primes.find_primes_until(upper);
    Ok((lower..=upper)
        .step_by(STEP_BY)
        .filter(|&n| !primes.is_prime(n))
        .count())
}

#[derive(Debug, Clone)]
struct Primes {
    found: Vec<u64>,
    half_upper: u64,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            found: {
                let mut v = Vec::with_capacity(1024);
                v.push(3);
                v
            },
            half_upper: 1,
        }
    }

    fn find_primes_until(&mut self, nr: u64) {
        let new_half_upper = nr >> 1;
        if self.half_upper >= new_half_upper {
            return;
        }
        let mut sqrt = (self.half_upper << 1 | 1).sqrt();
        let mut threshold = (sqrt + 1) * (sqrt + 1);
        'outer: for i in (self.half_upper + 1)..=new_half_upper {
            let n = i << 1 | 1;
            if n >= threshold {
                sqrt += 1;
                threshold = (sqrt + 1) * (sqrt + 1);
            }
            for &prime in &self.found {
                if prime > sqrt {
                    break;
                }
                if n % prime == 0 {
                    continue 'outer;
                }
            }

            self.found.push(n);
        }
        self.half_upper = new_half_upper;
    }

    fn is_prime(&mut self, nr: u64) -> bool {
        self.find_primes_until(nr);
        nr % 2 == 1 && self.found.binary_search(&nr).is_ok()
    }
}

fn parse(input: &[u8]) -> Result<Vec<Instruction>> {
    day18::parse(input)
}
