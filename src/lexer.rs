pub enum HtmlToken<'a> {
    Less,
    Greater,
    Slash,
    Text,
    Equal,
    Str(&'a str),
}
