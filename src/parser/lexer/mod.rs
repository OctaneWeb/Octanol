#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Segment {
    // our identifier '@' in this case
    Identifier(String),
    TagOpening(String),
    TagClosing(String),
    DocType(String),
    Literal(String),
}

pub type Segments = Vec<Segment>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Lexer {
    // add fields
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ContextCase {
    // witnessing a tag declaration
    Tag,
    // witnessing an attribute declaration
    Attribute,
    // inside a body we are
    Body,
}

// this is to add some more clarity to our Context here,
// tells about in which part in a context
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Event {
    AttributeDeclarationEvent,
    TagDeclarationEvent,
    // witnessing nothing, just in body or a tag
    Body,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Context {
    ctx: ContextCase,
    event: Event,
    literal: String,
}

impl Context {
    pub fn new() -> Self {
        // initially, assume the user is in the parent body
        Context {
            ctx: ContextCase::Body,
            event: Event::Body,
            literal: String::new(),
        }
    }
    pub fn set_context(&mut self, ctx: ContextCase) {
        self.ctx = ctx
    }
    pub fn set_event(&mut self, event: Event) {
        self.event = event
    }
    pub fn context_is(&self, ctx: ContextCase) -> bool {
        if self.ctx == ctx {
            true
        } else {
            false
        }
    }
    pub fn event_is(&self, event: Event) -> bool {
        if self.event == event {
            true
        } else {
            false
        }
    }
    pub fn push(&mut self, byte: char) {
        self.literal.push(byte)
    }
    pub fn break_literal_ctx(&mut self) -> String {
        let string = self.literal.clone();
        self.literal.clear();
        string
    }
    pub fn has_literal_context(&self) -> bool {
        if self.literal.len() == 0 || self.literal == "\n" {
            false
        } else {
            true
        }
    }
}

impl Lexer {
    pub fn lex(stream: Vec<u8>) -> Segments {
        use crate::parser::lexer::ContextCase::*;
        use crate::parser::lexer::Segment::*;
        let mut peekable = stream.into_iter().peekable();
        let mut lexemes: Segments = Vec::<Segment>::new();
        let mut ctx = Context::new();
        while peekable.peek().is_some() {
            match peekable.next() {
                Some(b'<') => {
                    if ctx.has_literal_context() {
                        lexemes.push(Literal(ctx.break_literal_ctx()));
                    }
                    ctx.push('<');
                    // <!DOCTYPE html> special case
                    if let Some(b'!') = peekable.peek() {
                        while let Some(x) = peekable.peek() {
                            match x {
                                b'\n' => {
                                    lexemes.push(DocType(ctx.break_literal_ctx()));
                                    ctx.push('\n');
                                    peekable.next();
                                    ctx.set_context(Body);
                                    break;
                                }
                                _ => ctx.push(peekable.next().unwrap() as char),
                            }
                        }
                    }
                    // if we were in a body previously, this is probably a tag declaration
                    if ctx.context_is(Body) {
                        ctx.set_context(Tag);
                        let mut tag_closing = false;
                        while let Some(x) = peekable.peek() {
                            match x {
                                // a space meaning the name of the tag has been finished lexing
                                b' ' => {
                                    ctx.push(peekable.next().unwrap() as char);
                                    ctx.set_event(Event::AttributeDeclarationEvent);
                                    if tag_closing {
                                        lexemes.push(TagClosing(ctx.break_literal_ctx()));
                                    } else {
                                        lexemes.push(TagOpening(ctx.break_literal_ctx()));
                                    }
                                    break;
                                }
                                // closing tag
                                b'/' => {
                                    peekable.next();
                                    tag_closing = true;
                                }
                                b'>' => {
                                    ctx.set_context(Body);
                                    ctx.push(peekable.next().unwrap() as char);
                                    if tag_closing {
                                        lexemes.push(TagClosing(ctx.break_literal_ctx()));
                                    } else {
                                        lexemes.push(TagOpening(ctx.break_literal_ctx()));
                                    }
                                    break;
                                }
                                _ => ctx.push(peekable.next().unwrap() as char),
                            }
                        }
                    }
                }
                Some(b'\n') => {
                    ctx.push('\n');
                }
                Some(b'@') => {
                    if ctx.context_is(Body) && ctx.event_is(Event::Body) {
                        let mut identifier_body = String::new();
                        while let Some(x) = peekable.peek() {
                            if *x == b' ' {
                                lexemes.push(Identifier(identifier_body.clone()));
                                identifier_body.clear();
                                break;
                            } else {
                                identifier_body.push(peekable.next().unwrap() as char)
                            }
                        }
                    } else {
                        ctx.push('@');
                    }
                }
                Some(x) => ctx.push(x as char),
                None => (),
            }
        }
        lexemes
    }
}
