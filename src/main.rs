use serde_json::{Error, Map, Number, Value};
use std::io::Read;

fn minimize(v: &Value) -> Value {
    match v {
        Value::Null => Value::Null,
        Value::Bool(_) => Value::Bool(false),
        Value::Number(_) => Value::Number(Number::from_f64(0 as f64).unwrap()),
        Value::String(_) => Value::String(String::from("")),
        Value::Array(arr) => {
            // Value doesn't implement Ord or Hash so i can't really think of a better way to do this
            // its kinda garbage but I think its the best way
            let arr = arr.iter().map(|x| minimize(x)).collect::<Vec<Value>>();
            let mut result: Vec<Value> = vec![];
            for i in arr {
                if !result.contains(&i) {
                    result.push(i.clone());
                }
            }
            Value::Array(result)
        }
        Value::Object(map) => {
            let mut result: Map<String, Value> = Map::new();
            for (key, value) in map
                .iter()
                .map(|(key, value)| (key.clone(), minimize(value)))
            {
                if !result.values().any(|x| x == &value) {
                    result.insert(key, value);
                }
            }
            Value::Object(result)
        }
    }
}

fn main() -> Result<(), Error> {
    let mut json = String::new();
    std::io::stdin()
        .read_to_string(&mut json)
        .expect("Input should be valid utf-8");

    let res = minimize(&serde_json::from_str(&json)?);

    println!("{}", serde_json::to_string_pretty(&res)?);
    Ok(())
}
