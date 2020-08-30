use crate::parser::lexer::{Segment, Segments};
pub mod lexer;

pub struct Parser;

impl Parser {
    pub fn parse(token_stream: Segments) {
        let mut peekable = token_stream.iter().peekable();
        while peekable.peek().is_some() {
            match peekable.next() {
                Some(Segment::Identifier(x)) => (),
                _ => (),
            }
        }
    }
}
