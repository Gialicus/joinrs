use anyhow::Result;
use std::{fs::File, io::Write, path::PathBuf};

use serde_json::{to_string_pretty, Value};

pub fn write_json(path: PathBuf, data: Vec<Value>) -> Result<()> {
    let json_str = to_string_pretty(&data)?;
    let mut file = File::create(path)?;
    file.write_all(json_str.as_bytes())?;
    Ok(())
}
