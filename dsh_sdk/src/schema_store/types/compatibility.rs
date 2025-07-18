use serde::{Deserialize, Serialize};

/// Schema compatibility level
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Compatibility {
    BACKWARD,
    BACKWARD_TRANSITIVE,
    FORWARD,
    FORWARD_TRANSITIVE,
    FULL,
    FULL_TRANSITIVE,
    NONE,
}

/// Schema config containing compatibility level
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigGet {
    pub compatibility_level: Compatibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Schema config containing compatibility level
///
/// For some reason the body is different compared from the get response
pub struct ConfigPut {
    pub compatibility: Compatibility,
}

/// Response from compatibility check
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CompatibilityCheck {
    pub is_compatible: bool,
}

impl CompatibilityCheck {
    pub fn is_compatible(&self) -> bool {
        self.is_compatible
    }
}

impl From<ConfigGet> for Compatibility {
    fn from(value: ConfigGet) -> Self {
        value.compatibility_level
    }
}

impl From<ConfigPut> for Compatibility {
    fn from(value: ConfigPut) -> Self {
        value.compatibility
    }
}

impl std::fmt::Display for Compatibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BACKWARD => write!(f, "BACKWARD"),
            Self::BACKWARD_TRANSITIVE => write!(f, "BACKWARD_TRANSITIVE"),
            Self::FORWARD => write!(f, "FORWARD"),
            Self::FORWARD_TRANSITIVE => write!(f, "FORWARD_TRANSITIVE"),
            Self::FULL => write!(f, "FULL"),
            Self::FULL_TRANSITIVE => write!(f, "FULL_TRANSITIVE"),
            Self::NONE => write!(f, "NONE"),
        }
    }
}
