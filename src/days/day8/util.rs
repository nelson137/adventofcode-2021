use std::fmt::Debug;

macro_rules! set {
    ($x:expr $(, $xs:expr)* $(,)?) => {{
        HashSet::from([$x, $($xs),*])
    }};
}
pub(super) use set;

pub trait ExpectIsolated<I: Iterator> {
    fn expect_isolated(self) -> I::Item;
}

impl<I> ExpectIsolated<I> for I
where
    I: Iterator,
    <I as Iterator>::Item: Debug + Copy,
{
    fn expect_isolated(mut self) -> I::Item {
        match self.next() {
            Some(answer) if self.next().is_none() => answer,
            _ => panic!("failed to isolate letter"),
        }
    }
}
