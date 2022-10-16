use std::io::Write;

use crate::{inputs::Inputs, result::Result};
use colored::Colorize;

pub fn run(days: &[Box<dyn AnyDayMetadata>]) -> Result<()> {
    println!(
        "{} {} {} {}",
        "Advent".underline().bright_red(),
        "of".underline().bright_yellow().bold(),
        "Code".underline().bright_green(),
        "2017".underline().bright_blue().bold()
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
    pub function: Box<dyn Fn(&T) -> Result<String>>,
}

impl<T> AnyDayMetadata for DayMetadata<T> {
    fn number(&self) -> u32 {
        self.number
    }
    fn execute(&self, inputs: &mut Inputs) -> Result<()> {
        println!(
            "{} {}",
            "Day".bright_green(),
            self.number.to_string().blue().bold()
        );

        let input = inputs.get(self.number)?;
        let parsed = (self.parse_fn)(&input)?;
        for part in &self.parts {
            print!("{}", part.name.bright_yellow().bold());
            _ = std::io::stdout().flush();
            let result = (part.function)(&parsed)?;
            if result.contains(|p| p == '\n') {
                println!("\n{result}");
            } else {
                println!(" {result}");
            }
        }

        Ok(())
    }
}

impl<T> SpecificDayMetadata for DayMetadata<T> {
    type Parsed = T;

    fn parse(&self, input: &[u8]) -> Result<Self::Parsed> {
        (self.parse_fn)(input)
    }
}
