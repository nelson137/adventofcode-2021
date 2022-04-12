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
        let i = s.find(char::is_whitespace).unwrap();
        let direction = &s[..i];
        let magnitude = (s.as_bytes()[i + 1] - '0' as u8) as isize;
        match direction {
            "forward" => Ok(Command::Forward(magnitude)),
            "up" => Ok(Command::Up(magnitude)),
            "down" => Ok(Command::Down(magnitude)),
            _ => Err(format!("direction not recognized: {}", direction).into()),
        }
    }
}
