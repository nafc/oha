extern crate liboha;
use liboha::lexer;
use liboha::parser;

fn main() {
    let test_data = r#"
0 + r"hey" + 'a'
true

let a = 1 + 10
a = '\n'

let b
b = "heyy"

let c = func(a, b)
    let c = a + b

    func add(a, b) func(a, b) a + b
"#;

    let mut block_tree = lexer::BlockTree::new(&test_data, 0);
    let indents        = block_tree.collect_indents();

    let lexed_root = lexer::lex_branch(&block_tree.tree(&indents));
    let flat_root  = lexer::flatten_branch(&lexed_root);

    println!("{:#?}\n, {:#?}", lexed_root, flat_root);

    let mut parser = parser::Parser::new(parser::Traveler::new(flat_root));

    println!("{:#?}", parser.parse());
}