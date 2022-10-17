use crate::{inputs::Inputs, prelude::ColoredOutput, result::Result};
use colored::Colorize;
use std::io::Write;

pub fn run(days: &[Box<dyn AnyDayMetadata>]) -> Result<()> {
    println!(
        "\n🎄 {} {} {} {} 🎄\n",
        "Advent".bright_red().bold(),
        "of".bright_green(),
        "Code".blue().bold(),
        "2017".white().bold()
    );

    let included_days: Vec<u32> = std::env::args()
        .filter_map(|v| v.parse::<u32>().ok())
        .collect();

    let mut inputs = Inputs::new();
    for day in days {
        if !included_days.is_empty() && !included_days.contains(&day.number()) {
            continue;
        }
        day.execute(&mut inputs)?;
    }
    println!();
    Ok(())
}

pub trait AnyDayMetadata {
    fn number(&self) -> u32;
    fn execute(&self, inputs: &mut Inputs) -> Result<()>;
}

pub trait SpecificDayMetadata: AnyDayMetadata {
    type Parsed;
    fn parse(&self, input: &[u8]) -> Result<Self::Parsed>;
}

pub struct DayMetadata<T> {
    pub number: u32,
    pub parse_fn: Box<dyn Fn(&[u8]) -> Result<T>>,
    pub parts: Vec<DayPart<T>>,
}

pub struct DayPart<T> {
    pub name: &'static str,
    pub function: Box<dyn Fn(&T) -> Result<ColoredOutput>>,
}

impl<T> AnyDayMetadata for DayMetadata<T> {
    fn number(&self) -> u32 {
        self.number
    }
    fn execute(&self, inputs: &mut Inputs) -> Result<()> {
        const OUTPUT_WIDTH: usize = 40;
        print!(
            "{} {}",
            "Day".bright_blue(),
            format!("{:>2}", self.number).bright_red().bold()
        );

        let input = inputs.get(self.number)?;
        let parsed = (self.parse_fn)(&input)?;
        for part in &self.parts {
            let remaining_space = OUTPUT_WIDTH.checked_sub(part.name.len() + 1).unwrap_or(0);
            print!(" :: {} ", part.name.bright_yellow());
            _ = std::io::stdout().flush();
            let result = (part.function)(&parsed)?;
            let str_len = result.value().len() - result.control_count();
            let remaining_space = remaining_space.checked_sub(str_len).unwrap_or(0);
            for _ in 0..remaining_space {
                print!(" ");
            }
            print!("{}", result.value());
            _ = std::io::stdout().flush();
        }
        println!();

        Ok(())
    }
}

impl<T> SpecificDayMetadata for DayMetadata<T> {
    type Parsed = T;

    fn parse(&self, input: &[u8]) -> Result<Self::Parsed> {
        (self.parse_fn)(input)
    }
}
