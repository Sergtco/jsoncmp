
use serde_json::Value;

pub fn compare_json(val1: &Value, val2: &Value) -> Vec<String> {
    match (val1, val2) {
        (Value::Object(m1), Value::Object(m2)) => m1
            .iter()
            .zip(m2.iter())
            .map(|(p1, p2)| {
                compare_json(p1.1, p2.1)
                    .iter()
                    .map(|mes| format!("{} -> {}", p1.0, mes))
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
mod test_cmp {
    use serde_json::json;

    use crate::cmp::compare_json;

    #[test]
    fn test_compare_json_simple() {
        let a = json!({
            "foo": "bar"
        });
        let b = json!({
            "foo": "foo"
        });
        assert_eq!(
            compare_json(&a, &b),
            vec!["foo -> \"bar\"!=\"foo\"".to_owned()]
        )
    }

    #[test]
    fn test_compare_json_nested() {
        let a = json!({
            "foo": {
                "bar": {
                    "zig": "zag"
                }
            }
        });
        let b = json!({
            "foo": {
                "bar": {
                    "zig": "zip"
                }
            }
        });
        assert_eq!(
            compare_json(&a, &b),
            vec!["foo -> bar -> zig -> \"zag\"!=\"zip\"".to_owned()]
        )
    }
    #[test]
    fn test_compare_json_multiple_err() {
        let a = json!({
            "foo": {
                "bar": {
                    "zig": "zag"
                },
                "dot": {
                    "dap": "dip"
                }
            }
        });
        let b = json!({
            "foo": {
                "bar": {
                    "zig": "zip"
                },
                "dot": {
                    "dap": "dup"
                }
            }
        });
        assert_eq!(
            compare_json(&a, &b),
            vec!["foo -> bar -> zig -> \"zag\"!=\"zip\"".to_owned(), "foo -> dot -> dap -> \"dip\"!=\"dup\"".to_owned()]
        )
    }
}
