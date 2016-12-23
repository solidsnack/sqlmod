use peg;

error_chain! {
    foreign_links {
        Parse(peg::ParseError);
        IO(::std::io::Error);
        UTF8(::std::str::Utf8Error);
    }
}
