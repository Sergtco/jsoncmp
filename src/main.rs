use core::panic;
use std::fs::read_to_string;
use std::str::FromStr;
use std::{env, process::ExitCode};

use serde_json::Value;

use jsoncmp::cmp;

fn main() -> Result<(), (ExitCode, &'static str)> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Unexpected amount of arguments: {}", args.len())
    }
    let (data1, data2) = (
        read_to_string(&args[1]).expect(&format!("Couldn't open file: {}", args[1])),
        read_to_string(&args[2]).expect(&format!("Couldn't open file: {}", args[2])),
    );

    let (json1, json2) = (
        Value::from_str(&data1).expect(&format!("Couldn't parse file: {}", args[1])),
        Value::from_str(&data2).expect(&format!("Couldn't parse file: {}", args[2])),
    );
    cmp::compare_json(&json1, &json2);
    Ok(())
}

