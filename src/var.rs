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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_extract_single_var() {
        let vars = extract_vars("docker run -p SINBO:port: myapp");
        assert_eq!(vars, vec!["port"]);
    }

    #[test]
    fn test_extract_multiple_vars() {
        let vars = extract_vars("docker run -p SINBO:port: -it SINBO:name:");
        assert_eq!(vars, vec!["port", "name"]);
    }

    #[test]
    fn test_extract_no_vars() {
        let vars = extract_vars("docker run -p 8080 myapp");
        assert!(vars.is_empty());
    }

    #[test]
    fn test_extract_empty_string() {
        let vars = extract_vars("");
        assert!(vars.is_empty());
    }

    #[test]
    fn test_extract_duplicate_vars() {
        let vars = extract_vars("SINBO:port: and SINBO:port: again");
        assert_eq!(vars, vec!["port", "port"]);
    }

    #[test]
    fn test_substitute_single_var() {
        let mut args = HashMap::new();
        args.insert("port".to_string(), "8080".to_string());
        let result = substitute("docker run -p SINBO:port:", &args).unwrap();
        assert_eq!(result, "docker run -p 8080");
    }

    #[test]
    fn test_substitute_multiple_vars() {
        let mut args = HashMap::new();
        args.insert("port".to_string(), "8080".to_string());
        args.insert("name".to_string(), "myapp".to_string());
        let result = substitute("docker run -p SINBO:port: -it SINBO:name:", &args).unwrap();
        assert_eq!(result, "docker run -p 8080 -it myapp");
    }

    #[test]
    fn test_substitute_no_vars() {
        let args = HashMap::new();
        let result = substitute("docker run -p 8080", &args).unwrap();
        assert_eq!(result, "docker run -p 8080");
    }

    #[test]
    fn test_substitute_missing_var_errors() {
        let args = HashMap::new();
        let result = substitute("docker run -p SINBO:port:", &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("port"));
    }

    #[test]
    fn test_substitute_empty_value() {
        let mut args = HashMap::new();
        args.insert("port".to_string(), "".to_string());
        let result = substitute("docker run -p SINBO:port:", &args).unwrap();
        assert_eq!(result, "docker run -p ");
    }

    #[test]
    fn test_substitute_preserves_non_placeholder_text() {
        let mut args = HashMap::new();
        args.insert("name".to_string(), "myapp".to_string());
        let result = substitute("prefix SINBO:name: suffix", &args).unwrap();
        assert_eq!(result, "prefix myapp suffix");
    }
}
