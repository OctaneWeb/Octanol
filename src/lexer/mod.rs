enum Token<'a> {
    Text(&'a str),
    Tag(Tag<'a>),
    Placeholder(&'a str),
    For(For<'a>),
    If(If<'a>),
}

type Tokens<'a> = Vec<Token<'a>>;

struct Tag<'a> {
    name: &'a str,
    slash: Slash,
    attrs: Vec<Attr<'a>>,
}

struct Attr<'a> {
    name: &'a str,
    value: AttrValue<'a>,
}

enum AttrValue<'a> {
    Placeholder(&'a str),
    Value(&'a str),
}

enum Slash {
    Left,
    Right,
    None,
}

struct For<'a> {
    var: &'a str,
    expr: &'a str,
    body: Tokens<'a>,
}

struct If<'a> {
    condition: Condition<'a>,
    body: Tokens<'a>,
    else_part: Option<Box<If<'a>>>,
}

enum Condition<'a> {
    Bool(&'a str),
    Let {
        pattern: &'a str,
        scrutinee: &'a str,
    },
}

pub struct Lexer {
    // add fields
}

impl Lexer {
    pub fn lex(stream: Vec<u8>) {
        let mut peekable = stream.iter().peekable();
        while peekable.peek().is_some() {
            match peekable.next() {
                Some(b'<') => (),
                Some(b'>') => (),
                _ => (),
            }
        }
    }
}
