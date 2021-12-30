macro_rules! repeat_char {
    ($c:expr, $n:expr) => {{
        (0..$n).map(|_| $c).collect::<String>()
    }};
}
pub(crate) use repeat_char;
