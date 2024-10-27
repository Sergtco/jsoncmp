use core::panic;
use std::fs::read_to_string;
use std::str::FromStr;
use std::{env, process::ExitCode};

use serde_json::Value;

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
    compare_json(&json1, &json2);
    Ok(())
}

fn compare_json(val1: &Value, val2: &Value) -> Vec<String> {
    match (val1, val2) {
        (Value::Object(m1), Value::Object(m2)) => m1
            .iter()
            .zip(m2.iter())
            .map(|(p1, p2)| {
                compare_json(p1.1, p2.1)
                    .iter()
                    .map(|mes| format!("In {}: {}", p1.0, mes))
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect(),
        (v1, v2) => match v1 == v2 {
            false => vec![format!("{}!={}", v1, v2)],
            true => vec![],
        },
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::compare_json;

    #[test]
    fn test_compare_json() {
        let a = json!({
            "foo": "bar"
        });
        let b = json!({
            "foo": "foo"
        });
        assert_eq!(compare_json(&a, &b), vec!["In foo: \"bar\"!=\"foo\"".to_owned()])
    }
}
