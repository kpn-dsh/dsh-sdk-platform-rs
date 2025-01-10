//! Add dead letter queue metadata to the kafka headers

use rdkafka::message::{Header, Headers, Message, OwnedHeaders, OwnedMessage};
use std::collections::HashMap;

use super::SendToDlq;

pub trait DlqHeaders {
    fn generate_dlq_headers<'a>(
        &'a self,
        dlq_message: &'a mut SendToDlq,
    ) -> HashMap<&'a str, Option<Vec<u8>>>;
}

impl DlqHeaders for OwnedMessage {
    fn generate_dlq_headers<'a>(
        &'a self,
        dlq_message: &'a mut SendToDlq,
    ) -> HashMap<&'a str, Option<Vec<u8>>> {
        let mut hashmap_headers: HashMap<&str, Option<Vec<u8>>> = HashMap::new();
        // Get original headers and add to hashmap
        if let Some(headers) = self.headers() {
            for header in headers.iter() {
                hashmap_headers.insert(header.key, header.value.map(|v| v.to_vec()));
            }
        }

        // Add dlq headers if not exist (we don't want to overwrite original dlq headers if message already failed earlier)
        let partition = self.partition().to_string().as_bytes().to_vec();
        let offset = self.offset().to_string().as_bytes().to_vec();
        let timestamp = self
            .timestamp()
            .to_millis()
            .unwrap_or(-1)
            .to_string()
            .as_bytes()
            .to_vec();
        hashmap_headers
            .entry("dlq_topic_origin")
            .or_insert_with(|| Some(self.topic().as_bytes().to_vec()));
        hashmap_headers
            .entry("dlq_partition_origin")
            .or_insert_with(move || Some(partition));
        hashmap_headers
            .entry("dlq_partition_offset_origin")
            .or_insert_with(move || Some(offset));
        hashmap_headers
            .entry("dlq_topic_origin")
            .or_insert_with(|| Some(self.topic().as_bytes().to_vec()));
        hashmap_headers
            .entry("dlq_timestamp_origin")
            .or_insert_with(move || Some(timestamp));
        // Overwrite if exist
        hashmap_headers.insert(
            "dlq_retryable",
            Some(dlq_message.retryable.to_string().as_bytes().to_vec()),
        );
        hashmap_headers.insert(
            "dlq_error",
            Some(dlq_message.error.to_string().as_bytes().to_vec()),
        );
        if let Some(stack_trace) = &dlq_message.stack_trace {
            hashmap_headers.insert("dlq_stack_trace", Some(stack_trace.as_bytes().to_vec()));
        }
        // update dlq_retries with +1 if exists, else add dlq_retries wiith 1
        let retries = hashmap_headers
            .get("dlq_retries")
            .map(|v| {
                let mut retries = [0; 4];
                retries.copy_from_slice(v.as_ref().unwrap());
                i32::from_be_bytes(retries)
            })
            .unwrap_or(0);
        hashmap_headers.insert("dlq_retries", Some((retries + 1).to_be_bytes().to_vec()));

        hashmap_headers
    }
}

pub trait HashMapToKafkaHeaders {
    fn to_owned_headers(&self) -> OwnedHeaders;
}

impl HashMapToKafkaHeaders for HashMap<&str, Option<Vec<u8>>> {
    fn to_owned_headers(&self) -> OwnedHeaders {
        // Convert to OwnedHeaders
        let mut owned_headers = OwnedHeaders::new_with_capacity(self.len());
        for header in self {
            let value = header.1.as_ref().map(|value| value.as_slice());
            owned_headers = owned_headers.insert(Header {
                key: header.0,
                value,
            });
        }
        owned_headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::dlq::tests::MockError;
    use crate::utils::dlq::types::*;
    use rdkafka::message::OwnedMessage;

    #[test]
    fn test_dlq_generate_dlq_headers() {
        let topic = "original_topic";
        let partition = 0;
        let offset = 123;
        let timestamp = 456;
        let error = Box::new(MockError::MockErrorRetryable("some_error".to_string()));

        let mut original_headers: OwnedHeaders = OwnedHeaders::new();
        original_headers = original_headers.insert(Header {
            key: "some_key",
            value: Some("some_value".as_bytes()),
        });

        let owned_message = OwnedMessage::new(
            Some(vec![1, 2, 3]),
            Some(vec![4, 5, 6]),
            topic.to_string(),
            rdkafka::Timestamp::CreateTime(timestamp),
            partition,
            offset,
            Some(original_headers),
        );

        let mut dlq_message = error.to_dlq(owned_message.clone());

        let mut expected_headers: HashMap<&str, Option<Vec<u8>>> = HashMap::new();
        expected_headers.insert("some_key", Some(b"some_value".to_vec()));
        expected_headers.insert("dlq_topic_origin", Some(topic.as_bytes().to_vec()));
        expected_headers.insert(
            "dlq_partition_origin",
            Some(partition.to_string().as_bytes().to_vec()),
        );
        expected_headers.insert(
            "dlq_partition_offset_origin",
            Some(offset.to_string().as_bytes().to_vec()),
        );
        expected_headers.insert(
            "dlq_timestamp_origin",
            Some(timestamp.to_string().as_bytes().to_vec()),
        );
        expected_headers.insert(
            "dlq_retryable",
            Some(Retryable::Retryable.to_string().as_bytes().to_vec()),
        );
        expected_headers.insert("dlq_retries", Some(1_i32.to_be_bytes().to_vec()));
        expected_headers.insert("dlq_error", Some(error.to_string().as_bytes().to_vec()));
        if let Some(stack_trace) = &dlq_message.stack_trace {
            expected_headers.insert("dlq_stack_trace", Some(stack_trace.as_bytes().to_vec()));
        }

        let result = owned_message.generate_dlq_headers(&mut dlq_message);
        for header in result.iter() {
            assert_eq!(
                header.1,
                expected_headers.get(header.0).unwrap_or(&None),
                "Header {} does not match",
                header.0
            );
        }

        // Test if dlq headers are correctly overwritten when to be retried message was already retried before
        let mut original_headers: OwnedHeaders = OwnedHeaders::new();
        original_headers = original_headers.insert(Header {
            key: "dlq_error",
            value: Some(
                "to_be_overwritten_error_as_this_was_the_original_error_from_1st_retry".as_bytes(),
            ),
        });
        original_headers = original_headers.insert(Header {
            key: "dlq_topic_origin",
            value: Some(topic.as_bytes()),
        });
        original_headers = original_headers.insert(Header {
            key: "dlq_retries",
            value: Some(&1_i32.to_be_bytes().to_vec()),
        });

        let owned_message = OwnedMessage::new(
            Some(vec![1, 2, 3]),
            Some(vec![4, 5, 6]),
            "retry_topic".to_string(),
            rdkafka::Timestamp::CreateTime(timestamp),
            partition,
            offset,
            Some(original_headers),
        );
        let result = owned_message.generate_dlq_headers(&mut dlq_message);
        assert_eq!(
            result.get("dlq_error").unwrap(),
            &Some(error.to_string().as_bytes().to_vec())
        );
        assert_eq!(
            result.get("dlq_topic_origin").unwrap(),
            &Some(topic.as_bytes().to_vec())
        );
        assert_eq!(
            result.get("dlq_retries").unwrap(),
            &Some(2_i32.to_be_bytes().to_vec())
        );
    }

    #[test]
    fn test_dlq_hashmap_to_owned_headers() {
        let mut hashmap: HashMap<&str, Option<Vec<u8>>> = HashMap::new();
        hashmap.insert("some_key", Some(b"key_value".to_vec()));
        hashmap.insert("some_other_key", None);
        let result: Vec<(&str, Option<&[u8]>)> =
            vec![("some_key", Some(b"key_value")), ("some_other_key", None)];

        let owned_headers = hashmap.to_owned_headers();
        for header in owned_headers.iter() {
            assert!(result.contains(&(header.key, header.value)));
        }
    }
}
