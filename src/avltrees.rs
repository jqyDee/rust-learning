use rust_learning::types::tree::avltree::AvlTree;

fn main() {
    let mut tree = AvlTree::new();
    tree.put(1, "first");
    tree.put(3, "third");
    tree.put(2, "second");
    tree.put(10, "tenth");
    tree.put(-1, "minus 1");
    tree.put(4, "fourth");
    tree.put(5, "fifth");

    tree.put(1, "1");
    tree.put(3, "3");
    tree.put(2, "2");
    tree.put(10, "10");
    tree.put(-1, "-1");
    tree.put(4, "4");
    tree.put(5, "5");
    println!("{}", tree);

    let mut str = String::new();
    tree.inorder(|node| {
        str += format!("{}, ", node).as_str();
    });

    println!("{}", str)
}
