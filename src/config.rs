use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct StateMachine {
    pub states: Vec<State>,
}

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

#[derive(Debug)]
pub enum Condition {
    Activity,        // issue was updated
    Command(String), // maintainer typed a given command in the comments
    Label(String),   // issue has a given label
    PullRequest,     // issue has a pull request attached resolving it
    Timeout(u16),    // issue was not updated for given number of days
}

#[derive(Debug)]
pub enum Action {
    AddLabel(String),
    Close,
    PostComment(String),
    ReplaceLabel(String),
    RemoveLabel(String),
}

// serde_helper defines structs that map to the config Schama,
// where Condition and Action are designed as union types,
// with a discriminating property `type`.
// The actual Condition and Action types in the main module
// are enums with custom deserialization and serialization.
mod serde_helper {
    use serde::{Deserialize, Serialize};

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

    // Meaningless, but needed to support Default for Condition
    impl Default for ConditionType {
        fn default() -> Self {
            ConditionType::Activity
        }
    }
    
    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Condition {
        #[serde(rename = "type")]
        pub condition_type: ConditionType,
        pub label: Option<String>,
        pub command: Option<String>,
        pub timeout: Option<u16>,
    }    

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ActionType {
        #[serde(rename = "add-label")]
        AddLabel,
        #[serde(rename = "close")]
        Close,
        #[serde(rename = "post-comment")]
        PostComment,
        #[serde(rename = "replace-label")]
        ReplaceLabel,
        #[serde(rename = "remove-label")]
        RemoveLabel,
    }

    // Meaningless, but needed to support Default for Action
    impl Default for ActionType {
        fn default() -> Self {
            ActionType::AddLabel
        }
    }
    
    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Action {
        #[serde(rename = "type")]
        pub action_type: ActionType,
        pub label: Option<String>,
        pub comment: Option<String>,
    }    
}

impl<'de> serde::Deserialize<'de> for Condition {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_helper::ConditionType;
        let condition: serde_helper::Condition = serde::Deserialize::deserialize(deserializer)?;
        match condition.condition_type {
            ConditionType::Activity => Ok(Condition::Activity),
            ConditionType::Command => Ok(Condition::Command(condition.command.unwrap())),
            ConditionType::Label => Ok(Condition::Label(condition.label.unwrap())),
            ConditionType::PullRequest => Ok(Condition::PullRequest),
            ConditionType::Timeout => Ok(Condition::Timeout(condition.timeout.unwrap())),
        }
    }
}

impl serde::Serialize for Condition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde_helper::ConditionType;
        match self {
            Condition::Activity => {
                let condition = serde_helper::Condition {
                    condition_type: ConditionType::Activity,
                    ..Default::default()
                };
                serde::Serialize::serialize(&condition, serializer)
            }
            Condition::Command(command) => {
                let condition = serde_helper::Condition {
                    condition_type: ConditionType::Command,
                    command: Some(command.to_string()),
                    ..Default::default()
                };
                serde::Serialize::serialize(&condition, serializer)
            }
            Condition::Label(label) => {
                let condition = serde_helper::Condition {
                    condition_type: ConditionType::Label,
                    label: Some(label.to_string()),
                    ..Default::default()
                };
                serde::Serialize::serialize(&condition, serializer)
            }
            Condition::PullRequest => {
                let condition = serde_helper::Condition {
                    condition_type: ConditionType::PullRequest,
                    label: None,
                    command: None,
                    timeout: None,
                };
                serde::Serialize::serialize(&condition, serializer)
            }
            Condition::Timeout(timeout) => {
                let condition = serde_helper::Condition {
                    condition_type: ConditionType::Timeout,
                    label: None,
                    command: None,
                    timeout: Some(*timeout),
                };
                serde::Serialize::serialize(&condition, serializer)
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_helper::ActionType;
        let action: serde_helper::Action = serde::Deserialize::deserialize(deserializer)?;
        match action.action_type {
            ActionType::AddLabel => Ok(Action::AddLabel(action.label.unwrap())),
            ActionType::Close => Ok(Action::Close), 
            ActionType::PostComment => Ok(Action::PostComment(action.comment.unwrap())),
            ActionType::ReplaceLabel => Ok(Action::ReplaceLabel(action.label.unwrap())),
            ActionType::RemoveLabel => Ok(Action::RemoveLabel(action.label.unwrap())),
        }
    }
}

// serialize Action
impl serde::Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde_helper::ActionType;
        match self {
            Action::AddLabel(label) => {
                let action = serde_helper::Action {
                    action_type: ActionType::AddLabel,
                    label: Some(label.to_string()),
                    comment: None,
                };
                serde::Serialize::serialize(&action, serializer)
            }
            Action::Close => {
                let action = serde_helper::Action {
                    action_type: ActionType::Close,
                    label: None,
                    comment: None,
                };
                serde::Serialize::serialize(&action, serializer)
            }
            Action::PostComment(comment) => {
                let action = serde_helper::Action {
                    action_type: ActionType::PostComment,
                    label: None,
                    comment: Some(comment.to_string()),
                };
                serde::Serialize::serialize(&action, serializer)
            }
            Action::ReplaceLabel(label) => {
                let action = serde_helper::Action {
                    action_type: ActionType::ReplaceLabel,
                    label: Some(label.to_string()),
                    comment: None,
                };
                serde::Serialize::serialize(&action, serializer)
            }
            Action::RemoveLabel(label) => {
                let action = serde_helper::Action {
                    action_type: ActionType::RemoveLabel,
                    label: Some(label.to_string()),
                    comment: None,
                };
                serde::Serialize::serialize(&action, serializer)
            }
        }
    }
}

