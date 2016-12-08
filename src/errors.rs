use peg;

error_chain! {
    foreign_links {
        Parse(peg::ParseError);
        IO(::std::io::Error);
    }
}
