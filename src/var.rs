use anyhow::{Result, anyhow};
use std::collections::HashMap;

const PREFIX: &str = "SINBO:";

pub fn extract_vars(content: &str) -> Vec<(String, Option<String>)> {
    let mut vars = Vec::new();
    let mut rest = content;

    while let Some(start) = rest.find(PREFIX) {
        rest = &rest[start + PREFIX.len()..];

        let Some(first_colon) = rest.find(':') else {
            break;
        };
        let name = rest[..first_colon].to_string();
        if name.is_empty() {
            rest = &rest[first_colon + 1..];
            continue;
        }
        rest = &rest[first_colon + 1..];

        let boundary = rest.find(PREFIX).unwrap_or(rest.len());
        let segment = &rest[..boundary];

        match segment.rfind(':') {
            None => {
                vars.push((name, None));
            }
            Some(0) => {
                rest = &rest[1..];
                vars.push((name, Some("".to_string())));
            }
            Some(last) => {
                let fallback = segment[..last].to_string();
                rest = &rest[last + 1..];
                vars.push((name, Some(fallback)));
            }
        }
    }

    vars
}

pub fn substitute(content: &str, args: &HashMap<String, String>) -> Result<String> {
    let vars = extract_vars(content);
    let mut result = content.to_string();

    for (name, fallback) in &vars {
        let placeholder = match fallback {
            Some(f) => format!("SINBO:{}:{}:", name, f),
            None => format!("SINBO:{}:", name),
        };

        let value = match args.get(name) {
            Some(val) => val.clone(),
            None => match fallback {
                Some(f) => f.clone(),
                None => return Err(anyhow!("missing value for variable '{}'", name)),
            },
        };

        result = result.replace(&placeholder, &value);
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
        assert_eq!(vars, vec![("port".to_string(), None)]);
    }

    #[test]
    fn test_extract_with_fallback() {
        let vars = extract_vars("docker run -p SINBO:port:8080: myapp");
        assert_eq!(vars, vec![("port".to_string(), Some("8080".to_string()))]);
    }

    #[test]
    fn test_extract_multiple_vars() {
        let vars = extract_vars("docker run -p SINBO:port: -it SINBO:name:");
        assert_eq!(
            vars,
            vec![("port".to_string(), None), ("name".to_string(), None),]
        );
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
    fn test_substitute_uses_fallback_when_missing() {
        let args = HashMap::new();
        let result = substitute("docker run -p SINBO:port:8080:", &args).unwrap();
        assert_eq!(result, "docker run -p 8080");
    }

    #[test]
    fn test_substitute_arg_overrides_fallback() {
        let mut args = HashMap::new();
        args.insert("port".to_string(), "9090".to_string());
        let result = substitute("docker run -p SINBO:port:8080:", &args).unwrap();
        assert_eq!(result, "docker run -p 9090");
    }

    #[test]
    fn test_substitute_missing_no_fallback_errors() {
        let args = HashMap::new();
        let result = substitute("docker run -p SINBO:port:", &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("port"));
    }

    #[test]
    fn test_substitute_no_vars() {
        let args = HashMap::new();
        let result = substitute("docker run -p 8080", &args).unwrap();
        assert_eq!(result, "docker run -p 8080");
    }

    #[test]
    fn test_substitute_preserves_non_placeholder_text() {
        let mut args = HashMap::new();
        args.insert("name".to_string(), "myapp".to_string());
        let result = substitute("prefix SINBO:name: suffix", &args).unwrap();
        assert_eq!(result, "prefix myapp suffix");
    }
    #[test]
    fn test_extract_fallback_with_colon_in_value() {
        let vars = extract_vars("SINBO:img:app:latest:");
        assert_eq!(
            vars,
            vec![("img".to_string(), Some("app:latest".to_string()))]
        );
    }

    #[test]
    fn test_extract_empty_fallback() {
        let vars = extract_vars("SINBO:name::");
        assert_eq!(vars, vec![("name".to_string(), Some("".to_string()))]);
    }

    #[test]
    fn test_extract_fallback_with_spaces() {
        let vars = extract_vars("SINBO:msg:hello world:");
        assert_eq!(
            vars,
            vec![("msg".to_string(), Some("hello world".to_string()))]
        );
    }
}
