use std::{error::Error, str::FromStr};

#[derive(PartialEq)]
pub enum Command {
    Forward(isize),
    Up(isize),
    Down(isize),
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let direction = match parts.next() {
            Some(d) => d,
            None => return Err("command has no direction".into()),
        };

        let magnitude: isize = match parts.next().and_then(|s| s.parse().ok()) {
            Some(m) => m,
            None => return Err(format!("invalid command: {}", s).into()),
        };

        match direction {
            "forward" => Ok(Command::Forward(magnitude)),
            "up" => Ok(Command::Up(magnitude)),
            "down" => Ok(Command::Down(magnitude)),
            _ => Err(format!("direction not recognized: {}", direction).into()),
        }
    }
}
