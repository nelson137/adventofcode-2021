#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum Height {
    Basin(Option<usize>),
    Border,
}

impl From<char> for Height {
    fn from(c: char) -> Self {
        if c == '9' {
            Self::Border
        } else {
            Self::Basin(None)
        }
    }
}
