use anyhow::{Error, Ok, Result};
use std::{fs::File, io::BufReader, path::PathBuf};

use serde_json::{Map, Value};

use crate::interface::cli::Cli;

fn read_json(path: PathBuf) -> Result<BufReader<File>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

#[test]
fn read_json_success() {
    assert_eq!(true, read_json("./test.json".into()).is_ok())
}

fn parse_json(reader: BufReader<File>) -> Result<Value> {
    let rdr = serde_json::from_reader(reader)?;
    Ok(rdr)
}

#[test]
fn parse_success() {
    let reader = read_json("./test.json".into()).unwrap();
    let result = parse_json(reader).unwrap();
    assert_eq!(result.is_array(), true)
}

pub fn parse(cli: &Cli) -> Result<(Value, Value)> {
    let s_reader = read_json(cli.source.clone())?;
    let s_result = parse_json(s_reader)?;
    let t_reader = read_json(cli.target.clone())?;
    let t_result = parse_json(t_reader)?;
    Ok((s_result, t_result))
}

pub fn join(cli: &Cli) -> Result<Vec<Value>> {
    let (s_result, t_result) = parse(cli)?;

    let s_result = s_result
        .as_array()
        .ok_or_else(|| Error::msg("JSON must be array"))?;
    let t_result = t_result
        .as_array()
        .ok_or_else(|| Error::msg("JSON must be array"))?;

    let mut result = Vec::new();

    for source_obj in s_result.iter().filter_map(|obj| obj.as_object()) {
        if let Some(source_value) = source_obj.get(&cli.column) {
            for target_obj in t_result.iter().filter_map(|obj| obj.as_object()) {
                if let Some(target_value) = target_obj.get(&cli.column) {
                    if target_value == source_value {
                        result.push(merge(
                            Value::from(source_obj.clone()),
                            Value::from(target_obj.clone()),
                        ))
                    }
                }
            }
        }
    }

    Ok(result)
}

pub fn merge(a: Value, b: Value) -> Value {
    let mut new_value = Map::new();
    if let Some(obj) = a.as_object() {
        for key in obj.keys() {
            new_value.insert(format!("s_{key}"), obj[key].clone());
        }
    }
    if let Some(obj) = b.as_object() {
        for key in obj.keys() {
            new_value.insert(format!("t_{key}"), obj[key].clone());
        }
    }
    Value::Object(new_value)
}

#[test]
fn should_merge_objects_with_different_keys() {
    let a = serde_json::json!({"key1": "value1"});
    let b = serde_json::json!({"key2": "value2"});
    let result = merge(a, b);
    let expected = serde_json::json!({"s_key1": "value1", "t_key2": "value2"});
    assert_eq!(result, expected);
}
