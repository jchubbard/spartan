pub enum Token <'a> {
    // Structural
    EOF, // end-of-file, dollar sign
    EOL, // end-of-line, semi-colon
    EOD, // end-of-delimited, comma

    // Logical
    While,
    For,
    If,

    // Scope
    Open(&'a str),
    Close(&'a str),

    // Types
    Var(&'a str),
    Val(u32),
    Type(&'a str),

    // Operations
    Op(&'a str),
    Func(&'a str)
}

pub struct TokenStream <'a> {
    stream: Vec<Token<'a>>
}