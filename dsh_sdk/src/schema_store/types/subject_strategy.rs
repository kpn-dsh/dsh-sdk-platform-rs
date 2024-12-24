use std::hash::{Hash, Hasher};

/// Subject name strategy
///
/// Defines the strategy to use for the subject name
///
/// ## Variants
/// Currently only the `TopicNameStrategy` is supported
///
/// - `TopicNameStrategy`: Use the topic name as the subject name and suffix of '-key' or '-value' for the key and value schemas
///
/// Example:
///```
/// # use dsh_sdk::schema_store::types::SubjectName;
/// SubjectName::TopicNameStrategy{topic: "scratch.example.tenant".to_string(), key: false}; // "scratch.example.tenant-value"
/// ```
#[derive(Debug, Clone, Eq)]
pub enum SubjectName {
    /// Use the topic name as the subject name and suffix of '-key' or '-value' for the key and value schemas
    ///
    /// Example:
    ///```
    /// # use dsh_sdk::schema_store::types::SubjectName;
    /// SubjectName::TopicNameStrategy{topic: "scratch.example.tenant".to_string(), key: false}; // "scratch.example.tenant-value"
    /// ```
    TopicNameStrategy { topic: String, key: bool },
}

impl SubjectName {
    pub fn new<S>(topic: S, key: bool) -> Self
    where
        S: AsRef<str>,
    {
        Self::TopicNameStrategy {
            topic: topic.as_ref().to_string(),
            key,
        }
    }
    pub fn name(&self) -> String {
        match self {
            Self::TopicNameStrategy { topic, key } => {
                if *key {
                    format!("{}-key", topic)
                } else {
                    format!("{}-value", topic)
                }
            }
        }
    }

    pub fn topic(self) -> String {
        match self {
            Self::TopicNameStrategy { topic, .. } => topic,
        }
    }
}

impl From<&str> for SubjectName {
    fn from(value: &str) -> Self {
        let (topic, key) = if value.ends_with("-key") {
            (value.trim_end_matches("-key"), true)
        } else if value.ends_with("-value") {
            (value.trim_end_matches("-value"), false)
        } else {
            (value, false)
        };
        Self::TopicNameStrategy {
            topic: topic.to_string(),
            key,
        }
    }
}

impl From<&String> for SubjectName {
    fn from(value: &String) -> Self {
        value.into()
    }
}

impl From<String> for SubjectName {
    fn from(value: String) -> Self {
        value.into()
    }
}

impl From<(&str, bool)> for SubjectName {
    fn from(value: (&str, bool)) -> Self {
        Self::TopicNameStrategy {
            topic: value.0.to_string(),
            key: value.1,
        }
    }
}

impl From<(String, bool)> for SubjectName {
    fn from(value: (String, bool)) -> Self {
        {
            Self::TopicNameStrategy {
                topic: value.0,
                key: value.1,
            }
        }
    }
}

impl std::fmt::Display for SubjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TopicNameStrategy { topic, key } => {
                write!(f, "{}-{}", topic, if *key { "key" } else { "value" })
            }
        }
    }
}

impl PartialEq for SubjectName {
    fn eq(&self, other: &SubjectName) -> bool {
        self.to_string() == other.to_string() // TODO: not the fastest way to compare, but it works for now
    }
}

impl Hash for SubjectName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}
