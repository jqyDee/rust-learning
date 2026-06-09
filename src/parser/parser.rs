use crate::types::CiArgs;
use std::{collections::HashSet, env::Args, net::Ipv4Addr};

pub fn parse_args(mut args: Args) -> Result<CiArgs, String> {
    let mut ci_args = CiArgs::new();
    let mut parsed_args = HashSet::new();

    args.next();

    while let Some(arg) = args.next() {
        if parsed_args.contains(&arg) {
            return Err(format!("Duplicate flag {}", arg));
        }

        match arg.as_str() {
            "--ip" => {
                let Some(next) = args.next() else {
                    return Err(format!("The {} flag expects a valid ip address", arg));
                };
                let Ok(ip) = next.parse::<Ipv4Addr>() else {
                    return Err(format!("{} is not a valid ip address", next));
                };
                parsed_args.insert(arg);
                ci_args.ip = ip;
            }
            "--port" => {
                let Some(next) = args.next() else {
                    return Err(format!("The {} flag expects a valid port", arg));
                };
                let Ok(port) = next.parse::<u16>() else {
                    return Err(format!("{} is not a valid u16 integer", next));
                };
                parsed_args.insert(arg);
                ci_args.port = port;
            }
            _ => return Err(String::from("Unknown argument!")),
        }
    }

    // TODO: check for the required fields. None so far.

    Ok(ci_args)
}
