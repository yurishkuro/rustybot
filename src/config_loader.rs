use super::config;

#[derive(Debug)]
enum ConfigError {
    SchemaLoading(String),
    SchemaValidation(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::SchemaLoading(msg) => write!(f, "Schema loading error: {}", msg),
            ConfigError::SchemaValidation(msg) => {
                write!(f, "Schema validation error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

const SCHEMA_BYTES: &[u8] = include_bytes!("config-schema.json");

fn load_schema(schema_bytes: &[u8]) -> Result<jsonschema::JSONSchema, ConfigError> {
    use jsonschema::{Draft, JSONSchema};
    use serde_json;

    let schema_str = unsafe { std::str::from_utf8_unchecked(schema_bytes) };
    let schema_json: serde_json::Value =
        serde_json::from_str(schema_str).map_err(|err| -> ConfigError {
            println!("Schema Parsing Error: {}", err);
            ConfigError::SchemaLoading(err.to_string())
        })?;

    return JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_json)
        .map_err(|err| -> ConfigError {
            println!("Schema Compile Error: {}", err);
            ConfigError::SchemaLoading(err.to_string())
        });
}

fn validate_config(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let schema = load_schema(SCHEMA_BYTES)?;

    let yaml_content = std::fs::read_to_string(file_name)?;
    let yaml_value: serde_json::Value = serde_yaml::from_str(&yaml_content)?;

    Ok(schema.validate(&yaml_value).map_err(|errors| {
        let mut out = String::from("Validation Errors: ");
        for error in errors {
            out.push_str("\n  - ");
            out.push_str(&error.to_string());
        }
        Box::new(ConfigError::SchemaValidation(out))
    })?)
}

pub fn load_config(file_name: &str) -> Result<config::StateMachine, Box<dyn std::error::Error>> {
    use serde_yaml;
    validate_config(file_name)?;
    let yaml_content = std::fs::read_to_string(file_name)?;
    let config: config::StateMachine = serde_yaml::from_str(&yaml_content)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let yaml = r#"
        states:
          - description: "Issue is open"
            label: "open"
            transitions:
              - description: "Issue is stale"
                conditions:
                  - type: "timeout"
                    timeout: 10
                actions:
                  - type: "add-label"
                    label: "stale"
              - description: "Issue is resolved"
                conditions:
                  - type: "label"
                    label: "resolved"
                actions:
                  - type: "close"
          - description: "Issue is stale"
            label: "stale"
            transitions:
              - description: "Issue is resolved"
                conditions:
                  - type: "label"
                    label: "resolved"
                actions:
                  - type: "close"
        "#;
        let mut file = NamedTempFile::new().expect("Failed to create temporary file");
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let config = load_config(file.path().to_str().unwrap()).unwrap();
        assert!(config.states.len() == 2);
        assert!(config.states[0].transitions.len() == 2);
        assert!(config.states[1].transitions.len() == 1);
    }
}
