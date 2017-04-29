extern crate liboha;
use liboha::lexer;

fn main() {
    let mut test_data = r#"
ident?
ident_
ident123
_ident

1 + 1 * 20 - .123 * 11.11
r"hey\n"
"hey\n"
'c'
'\n'
    "#.chars();

    let lexer = lexer::lexer(&mut test_data);

    for t in lexer {
        println!("{:#?}", t)
    }
}
