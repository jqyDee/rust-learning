use rust_learning::types::{BTree, math_token::MathToken};

fn main() {
    let mut t: BTree<char> = BTree::new();
    let root = t.add_root('+');
    root.insert_left('1');
    root.insert_right('2');

    println!("tree: {:?}", t);

    let mut mt: BTree<MathToken> = BTree::new();
    let root = mt.add_root(MathToken::Add);
    root.insert_left(MathToken::Number(1.0));
    let right = root.insert_right(MathToken::Multiply);
    right.insert_left(MathToken::Number(2.0));
    right.insert_right(MathToken::Number(3.0));
    println!("tree: {}", mt);
    println!("eval: {}", mt.evaluate());
}
