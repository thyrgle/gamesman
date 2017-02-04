use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(PartialEq, Debug)]
pub enum Status {
    Win,
    Loss,
    Tie,
    Undecided
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
