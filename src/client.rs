use rust_learning::parser::parse_args;
use std::env;

fn main() -> Result<(), ()> {
    println!("Hello from Client");

    let _args_parsed =
        parse_args(env::args()).map_err(|e| eprintln!("Could not parse args: {}", e))?;

    Ok(())
}
