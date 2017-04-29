extern crate liboha;
use liboha::lexer;

fn main() {
    let test_data = r#"
func foo(a, b)
    if true == false
        pass a + b
    else
        pass a - b
"#;

    let mut block_tree = lexer::BlockTree::new(&test_data, 0);
    let indents        = block_tree.collect_indents();

    let lexed_root = lexer::lex_branch(&block_tree.tree(&indents));
    let flat_root  = lexer::flatten_branch(&lexed_root);

    println!("{:#?}\n, {:#?}", lexed_root, flat_root)
}
