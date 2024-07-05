use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub description: String,
    pub label: String,
    pub transitions: Vec<Transition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transition {
    pub description: String,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ConditionType {
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "activity")]
    Activity,
    #[serde(rename = "pull-request")]
    PullRequest,
    #[serde(rename = "command")]
    Command,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    #[serde(rename = "type")]
    pub condition_type: ConditionType,
    pub label: Option<String>,
    pub command: Option<String>,
    pub timeout: Option<u32>,
    pub linked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionType {
    #[serde(rename = "add-label")]
    AddLabel,
    #[serde(rename = "replace-label")]
    ReplaceLabel,
    #[serde(rename = "remove-label")]
    RemoveLabel,
    #[serde(rename = "post-comment")]
    PostComment,
    #[serde(rename = "close")]
    Close,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: ActionType,
    pub label: Option<String>,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StateMachineConfig {
    pub states: Vec<State>,
}

// #[derive(JsonSchema)]
// struct StateMachineConfig {
//     // Add your config fields here
// }

#[derive(Debug)]
enum ConfigError {
    // IoError(std::io::Error),
    SchemaLoadingError(String),
    SchemaValidationError(String),
    // YamlError(serde_yaml::Error),
    // SchemaError(jsonschema::ValidationError<'a>),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::SchemaLoadingError(msg) => write!(f, "Schema loading error: {}", msg),
            ConfigError::SchemaValidationError(msg) => write!(f, "Schema validation error: {}", msg),
            // ...
        }
    }
}

impl std::error::Error for ConfigError {}

const SCHEMA_BYTES: &[u8] = include_bytes!("config-schema.json");

fn load_schema(schema_bytes: &[u8]) -> Result<jsonschema::JSONSchema, ConfigError> {
    use jsonschema::{Draft, JSONSchema};
    use serde_json;

    let schema_str = unsafe { std::str::from_utf8_unchecked(schema_bytes) };
    let schema_json: serde_json::Value = serde_json::from_str(schema_str)
        .map_err(|err|-> ConfigError {
            println!("Schema Parsing Error: {}", err);
            ConfigError::SchemaLoadingError(err.to_string())
        })?;

    return JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_json).map_err(|err| -> ConfigError {
            println!("Schema Compile Error: {}", err);
            ConfigError::SchemaLoadingError(err.to_string())
        });
}

pub fn validate_config(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let schema = load_schema(SCHEMA_BYTES)?;

    let yaml_content = std::fs::read_to_string(file_name)?;
    let yaml_value: serde_json::Value = serde_yaml::from_str(&yaml_content)?;

    Ok(schema.validate(&yaml_value).map_err(|errors| {
        let mut out = String::from("Validation Errors: ");
        for error in errors {
            out.push('\n');
            out.push_str(&error.to_string());
        }
        Box::new(ConfigError::SchemaValidationError(out))
    })?)
}

pub fn load_config(file_name: &str) -> Result<StateMachineConfig, Box<dyn std::error::Error>> {
    use serde_yaml;

    validate_config(file_name)?;

    let yaml_content = std::fs::read_to_string(file_name)?;
    let config: StateMachineConfig = serde_yaml::from_str(&yaml_content)?;

    // // Validate the YAML value against the schema
    // let validation_result = schema.validate(&yaml_value);
    // match validation_result {
    //     Ok(_) => println!("YAML data is valid!"),
    //     Err(errors) => {
    //         let mut out = String::from("Validation Errors: ");
    //         for error in errors {
    //             out.push('\n');
    //             out.push_str(&error.to_string());
    //         }
    //         return Err(Box::new(ConfigError::SchemaValidationError(out)));
    //     }
    // }

    // // let file = File::open(file_name).map_err(ConfigError::IoError)?;
    // // let reader = BufReader::new(file);

    // let config = StateMachineConfig { states: Vec::new() }; // = serde_yaml::from_reader(reader).unwrap()?;
    Ok(config)
}
