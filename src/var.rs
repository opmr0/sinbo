use anyhow::{Result, anyhow};
use std::collections::HashMap;

const PREFIX: &str = "SINBO:";

pub fn extract_vars(content: &str) -> Vec<String> {
    let mut vars = Vec::new();
    let mut rest = content;

    while let Some(start) = rest.find(PREFIX) {
        rest = &rest[start + PREFIX.len()..];
        if let Some(end) = rest.find(':') {
            vars.push(rest[..end].to_string());
            rest = &rest[end + 1..];
        }
    }

    vars
}

pub fn substitute(content: &str, args: &HashMap<String, String>) -> Result<String> {
    let vars = extract_vars(content);
    let mut result = content.to_string();

    for var in &vars {
        let placeholder = format!("SINBO:{}:", var);
        match args.get(var) {
            Some(val) => result = result.replace(&placeholder, val),
            None => return Err(anyhow!("missing value for variable '{}'", var)),
        }
    }

    Ok(result)
}
