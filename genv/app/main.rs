use super::error::PineResult;
use super::input::Input;
use super::utils::input_end;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::combinator::{recognize};
use nom::sequence::tuple;
#[cfg(test)]
mod tests {
    use super::super::input::Position;
    use super::*;
    use crate::ast::error::PineError;
    use nom::{
        error::{ErrorKind, ParseError},
        Err,
    };
    use std::convert::TryInto;

    #[test]
    fn comment_test() {
        let s = Input::new_with_str("//hello world\nwode");
        assert_eq!(
            comment(s),
            Ok((
                Input::new("wode", Position::new(1, 0), Position::max()),
                Input::new_u32("//hello world\n", 0, 0, 1, 0)
            ))
        );
    }
}