use std::hash::{Hash, Hasher};

use crate::schema_store::SchemaStoreError;

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
    pub fn new_topic_name_strategy<S>(topic: S, key: bool) -> Self
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

    pub fn topic(&self) -> &str {
        match self {
            Self::TopicNameStrategy { topic, .. } => topic,
        }
    }

    pub fn key(&self) -> bool {
        match self {
            Self::TopicNameStrategy { key, .. } => *key,
        }
    }
}

impl TryFrom<&str> for SubjectName {
    type Error = SchemaStoreError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.ends_with("-key") {
            Ok(Self::TopicNameStrategy {
                topic: value.trim_end_matches("-key").to_string(),
                key: true,
            })
        } else if value.ends_with("-value") {
            Ok(Self::TopicNameStrategy {
                topic: value.trim_end_matches("-value").to_string(),
                key: false,
            })
        } else {
            Err(SchemaStoreError::InvalidSubjectName(value.to_string()))
        }
    }
}

impl TryFrom<String> for SubjectName {
    type Error = SchemaStoreError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::DefaultHasher;

    #[test]
    fn test_subject_name_funcitons() {
        let subject = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: false,
        };
        assert_eq!(subject.topic(), "scratch.example.tenant");
        assert_eq!(subject.key(), false);
        assert_eq!(subject.name(), "scratch.example.tenant-value");

        let subject = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: true,
        };
        assert_eq!(subject.topic(), "scratch.example.tenant");
        assert_eq!(subject.key(), true);
        assert_eq!(subject.name(), "scratch.example.tenant-key");
    }

    #[test]
    fn test_subject_name_new() {
        let subject = SubjectName::new_topic_name_strategy("scratch.example.tenant", false);
        assert_eq!(
            subject,
            SubjectName::TopicNameStrategy {
                topic: "scratch.example.tenant".to_string(),
                key: false
            }
        );
    }

    #[test]
    fn test_subject_name_from_string() {
        let subject: SubjectName = "scratch.example.tenant-value".try_into().unwrap();
        assert_eq!(
            subject,
            SubjectName::TopicNameStrategy {
                topic: "scratch.example.tenant".to_string(),
                key: false
            }
        );

        let subject: SubjectName = "scratch.example.tenant-key".try_into().unwrap();
        assert_eq!(
            subject,
            SubjectName::TopicNameStrategy {
                topic: "scratch.example.tenant".to_string(),
                key: true
            }
        );
    }

    #[test]
    fn test_subject_name_from_tuple() {
        let subject: SubjectName = ("scratch.example.tenant".to_string(), false).into();
        assert_eq!(
            subject,
            SubjectName::TopicNameStrategy {
                topic: "scratch.example.tenant".to_string(),
                key: false
            }
        );

        let subject: SubjectName = ("scratch.example.tenant".to_string(), true).into();
        assert_eq!(
            subject,
            SubjectName::TopicNameStrategy {
                topic: "scratch.example.tenant".to_string(),
                key: true
            }
        );
    }

    #[test]
    fn test_subject_name_from_string_ref() {
        let string = "scratch.example.tenant-value".to_string();
        let subject: SubjectName = string.try_into().unwrap();
        assert_eq!(
            subject,
            SubjectName::TopicNameStrategy {
                topic: "scratch.example.tenant".to_string(),
                key: false
            }
        );
    }

    #[test]
    fn test_subject_name_display() {
        let subject = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: false,
        };
        assert_eq!(subject.to_string(), "scratch.example.tenant-value");

        let subject = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: true,
        };
        assert_eq!(subject.to_string(), "scratch.example.tenant-key");
    }

    #[test]
    fn test_subject_name_eq() {
        let subject1 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: false,
        };
        let subject2 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: false,
        };
        assert_eq!(subject1, subject2);

        let subject1 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: true,
        };
        let subject2 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: true,
        };
        assert_eq!(subject1, subject2);

        let subject1 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: false,
        };
        let subject2 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: true,
        };
        assert_ne!(subject1, subject2);
    }

    #[test]
    fn test_subject_name_hash() {
        let subject1 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: false,
        };
        let subject2 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: false,
        };
        let mut hasher = DefaultHasher::new();
        subject1.hash(&mut hasher);
        let hash1 = hasher.finish();
        let mut hasher = DefaultHasher::new();
        subject2.hash(&mut hasher);
        let hash2 = hasher.finish();
        assert_eq!(hash1, hash2);

        let subject1 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: true,
        };
        let subject2 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: true,
        };
        let mut hasher = DefaultHasher::new();
        subject1.hash(&mut hasher);
        let hash1 = hasher.finish();
        let mut hasher = DefaultHasher::new();
        subject2.hash(&mut hasher);
        let hash2 = hasher.finish();
        assert_eq!(hash1, hash2);

        let subject1 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: false,
        };
        let subject2 = SubjectName::TopicNameStrategy {
            topic: "scratch.example.tenant".to_string(),
            key: true,
        };
        let mut hasher = DefaultHasher::new();
        subject1.hash(&mut hasher);
        let hash1 = hasher.finish();
        let mut hasher = DefaultHasher::new();
        subject2.hash(&mut hasher);
        let hash2 = hasher.finish();
        assert_ne!(hash1, hash2);
    }
}
