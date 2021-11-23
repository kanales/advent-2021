pub enum AdventError {
    ParseError,
    ExecutionError,
    EofError,
    UnknownDay(i32),
}

pub type AdventResult = Result<i32, AdventError>;

pub trait Puzzle {
    fn first(&self) -> AdventResult;
    fn second(&self) -> AdventResult;
}
