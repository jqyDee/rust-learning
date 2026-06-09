use rust_learning::types::{BNode, BTree, math_token::MathToken};

fn main() {
    let mut args = std::env::args();
    let _prog_name = args.next().unwrap();
    let input = args.next().unwrap();

    if let Ok(tokens) = tokenize(input.as_str()) {
        if let Ok(tree) = parse_to_tree(tokens) {
            println!("tree size: {}", tree.size);
            println!("formula parsed: {}", tree);
            println!("eval: {}", tree.evaluate());
        }
    }
}

fn tokenize(input: &str) -> Result<Vec<MathToken>, String> {
    let mut tokens = Vec::new();

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.peek() {
        match c {
            // skip all the whitespace character
            ' ' | '\n' | '\t' => {
                chars.next();
            }

            // handle operators
            '+' => {
                tokens.push(MathToken::Add);
                chars.next();
            }
            '-' => {
                tokens.push(MathToken::Subtract);
                chars.next();
            }
            '*' => {
                tokens.push(MathToken::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(MathToken::Divide);
                chars.next();
            }
            '(' => {
                tokens.push(MathToken::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(MathToken::RightParen);
                chars.next();
            }

            // numbers
            '0'..='9' | '.' => {
                let mut num_str = String::new();

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_digit() || next_char == '.' {
                        num_str.push(next_char);
                        chars.next(); // Consume the digit
                    } else {
                        break; // Stop when we hit a non-number (like a '+')
                    }
                }

                match num_str.parse::<f64>() {
                    Ok(num) => tokens.push(MathToken::Number(num)),
                    Err(_) => return Err(format!("Invalid number format: '{}'", num_str)),
                }
            }

            _ => return Err(format!("Unexpected char: {}", c)),
        }
    }

    Ok(tokens)
}

fn parse_to_tree(tokens: Vec<MathToken>) -> Result<BTree<MathToken>, String> {
    let mut nodes: Vec<Box<BNode<MathToken>>> = Vec::new();
    let mut operators: Vec<MathToken> = Vec::new();

    let apply_operator = |nds: &mut Vec<Box<BNode<MathToken>>>, ops: &mut Vec<MathToken>| {
        let op = ops.pop().expect("Formula invalid");
        let right = nds.pop().expect("Formula invalid");
        let left = nds.pop().expect("Formula invalid");

        let mut par = Box::new(BNode::new(op));
        par.left = Some(left);
        par.right = Some(right);
        par.leaf = false;

        nds.push(par);
    };

    for token in tokens {
        match token {
            // simply push onto the node/op stack
            MathToken::Number(_) => {
                nodes.push(Box::new(BNode::new(token)));
            }
            MathToken::LeftParen => {
                operators.push(token);
            }

            // actually create the nodes
            MathToken::RightParen => {
                while let Some(top) = operators.last() {
                    if *top == MathToken::LeftParen {
                        break;
                    }
                    apply_operator(&mut nodes, &mut operators);
                }
                operators.pop();
            }
            _ => {
                while let Some(top) = operators.last() {
                    if top.precedence() >= token.precedence() {
                        apply_operator(&mut nodes, &mut operators);
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
        }
    }

    while !operators.is_empty() {
        apply_operator(&mut nodes, &mut operators);
    }

    if nodes.len() == 1 {
        let root = nodes.pop().unwrap();
        Ok(BTree::from(root))
    } else {
        Err("Syntax error: mismatched operators or numbers".to_string())
    }
}
