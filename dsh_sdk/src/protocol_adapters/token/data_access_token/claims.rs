use serde::{Deserialize, Serialize};

/// Permissions per topic for the [`DataAccessToken`](super::DataAccessToken).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct TopicPermission {
    /// Publish or Subscribe
    action: Action,
    /// The resource to define what the client can access in terms of stream, prefix, topic, and type.
    resource: Resource,
}

/// `publish` or `subscribe` permisison for [`TopicPermission`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, Hash)]
pub enum Action {
    #[serde(alias = "publish")]
    Publish,
    #[serde(alias = "subscribe")]
    Subscribe,
}

/// Represents a resource/datastream in the [`TopicPermission`] claim.
///
/// The resource defines what the client can access in terms of stream, prefix, topic, and type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
struct Resource {
    /// The type of the resource (always "topic").
    #[serde(rename = "type")]
    resource_type: String,
    /// data stream name, e.g. weather or ivi
    stream: String,
    /// topic prefix, e.g. /tt
    prefix: String,
    /// topic pattern, e.g. +/+/+/something/#
    topic: String,
}

impl TopicPermission {
    /// Creates a new [`TopicPermission`] instance.
    ///
    /// # Arguments
    ///
    /// * `resource` - the resource to define what the client can access in terms of stream, prefix, topic, and type.
    /// * `action` - the action to define what the client can do with the resource.
    ///
    /// # Returns
    ///
    /// Returns a new [`TopicPermission`] instance.
    pub fn new(
        action: Action,
        stream: impl Into<String>,
        prefix: impl Into<String>,
        topic_pattern: impl Into<String>,
    ) -> Self {
        let resource = Resource::new(stream, prefix, topic_pattern);
        Self { resource, action }
    }

    /// Returns the full qualified topic name of resource.
    pub fn full_qualified_topic_name(&self) -> String {
        format!(
            "{}/{}/{}",
            self.resource.prefix, self.resource.stream, self.resource.topic
        )
    }

    /// topic prefix, e.g. `/tt`
    pub fn prefix(&self) -> &str {
        &self.resource.prefix
    }

    /// data stream name, e.g. `weather` or `ivi`
    pub fn stream(&self) -> &str {
        &self.resource.stream
    }

    /// topic pattern, e.g. `+/+/+/something/#`
    pub fn topic_pattern(&self) -> &str {
        &self.resource.topic
    }

    /// Returns the [`Action`] to define what the client can do with the resource.
    pub fn action(&self) -> Action {
        self.action
    }
}

impl Resource {
    /// Creates a new [`Resource`] instance.
    ///
    /// # Arguments
    ///
    /// * `stream` - data stream name, e.g. `weather` or `ivi`
    /// * `prefix` - topic prefix, e.g.`/tt`
    /// * `topic` - topic pattern, e.g. `+/+/+/something/#`
    ///
    /// # Returns
    ///
    /// Returns a new [`Resource`] instance.
    pub fn new(
        stream: impl Into<String>,
        prefix: impl Into<String>,
        topic_pattern: impl Into<String>,
    ) -> Self {
        Self {
            stream: stream.into(),
            prefix: prefix.into(),
            topic: topic_pattern.into(),
            resource_type: "topic".to_string(), // always topic
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Publish => write!(f, "publish"),
            Self::Subscribe => write!(f, "subscribe"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_permission_new() {
        let topic_permission = TopicPermission::new(Action::Publish, "stream", "prefix", "topic/#");
        assert_eq!(topic_permission.action(), Action::Publish);
        assert_eq!(topic_permission.stream(), "stream");
        assert_eq!(topic_permission.prefix(), "prefix");
        assert_eq!(topic_permission.topic_pattern(), "topic/#");
    }

    #[test]
    fn test_resource_new() {
        let resource = Resource::new("stream", "prefix", "topic/#");
        assert_eq!(resource.stream, "stream");
        assert_eq!(resource.prefix, "prefix");
        assert_eq!(resource.topic, "topic/#");
    }

    #[test]
    fn test_action_display() {
        let action = Action::Publish;
        assert_eq!(action.to_string(), "publish");
        let action = Action::Subscribe;
        assert_eq!(action.to_string(), "subscribe");
    }
}
