use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum NavChunk {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    AngledOpen,
    AngledClose,
}

impl TryFrom<char> for NavChunk {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use NavChunk::*;
        match value {
            '(' => Ok(RoundOpen),
            ')' => Ok(RoundClose),
            '[' => Ok(SquareOpen),
            ']' => Ok(SquareClose),
            '{' => Ok(CurlyOpen),
            '}' => Ok(CurlyClose),
            '<' => Ok(AngledOpen),
            '>' => Ok(AngledClose),
            _ => Err(format!("invalid chunk type: {}", value).into()),
        }
    }
}

impl NavChunk {
    pub fn syntax_error_score(&self) -> Result<usize, Box<dyn Error>> {
        use NavChunk::*;
        match *self {
            RoundClose => Ok(3),
            SquareClose => Ok(57),
            CurlyClose => Ok(1197),
            AngledClose => Ok(25137),
            _ => Err(format!("chunk end has no score: {:?}", self).into()),
        }
    }

    pub fn completion_score(&self) -> Result<usize, Box<dyn Error>> {
        use NavChunk::*;
        match *self {
            RoundClose => Ok(1),
            SquareClose => Ok(2),
            CurlyClose => Ok(3),
            AngledClose => Ok(4),
            _ => Err(format!("chunk end has no score: {:?}", self).into()),
        }
    }
}
