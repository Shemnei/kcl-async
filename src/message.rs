pub mod output {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "action")]
    pub enum Message {
        #[serde(rename = "checkpoint")]
        Checkpoint(CheckpointMessage),

        #[serde(rename = "status")]
        Status(StatusMessage),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CheckpointMessage {
        #[serde(rename = "sequenceNumber")]
        pub sequence_number: Option<String>,

        #[serde(
            rename = "subSequenceNumber",
            skip_serializing_if = "Option::is_none",
            default
        )]
        pub sub_sequence_number: Option<u64>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StatusMessage {
        #[serde(rename = "responseFor")]
        pub response_for: String,
    }

    impl StatusMessage {
        pub fn from_message(message: &super::input::Message) -> Self {
            Self {
                response_for: message.id().into(),
            }
        }
    }
}

pub mod input {
    use base64::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "action")]
    pub enum Message {
        #[serde(rename = "checkpoint")]
        Checkpoint(CheckpointMessage),

        #[serde(rename = "initialize")]
        Initialize(InitializeMessage),

        #[serde(rename = "processRecords")]
        ProcessRecords(ProcessRecordsMessage),

        #[serde(rename = "shutdown")]
        Shutdown(ShutdownMessage),

        #[serde(rename = "shutdownRequested")]
        ShutdownRequested(ShutdownRequestedMessage),

        #[serde(rename = "leaseLost")]
        LeaseLost(LeaseLostMessage),

        #[serde(rename = "shardEnded")]
        ShardEnded(ShardEndedMessage),
    }

    impl Message {
        pub const fn id(&self) -> &'static str {
            match self {
                Message::Checkpoint(_) => "checkpoint",
                Message::Initialize(_) => "initialize",
                Message::ProcessRecords(_) => "processRecords",
                Message::Shutdown(_) => "shutdown",
                Message::ShutdownRequested(_) => "shutdownRequested",
                Message::LeaseLost(_) => "leaseLost",
                Message::ShardEnded(_) => "shardEnded",
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CheckpointMessage {
        #[serde(rename = "sequenceNumber")]
        pub sequence_number: Option<String>,

        #[serde(
            rename = "subSequenceNumber",
            skip_serializing_if = "Option::is_none",
            default
        )]
        pub sub_sequence_number: Option<u64>,

        #[serde(rename = "error", skip_serializing_if = "Option::is_none", default)]
        pub error: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializeMessage {
        #[serde(rename = "shardId")]
        pub shard_id: String,

        #[serde(
            rename = "sequenceNumber",
            skip_serializing_if = "Option::is_none",
            default
        )]
        pub sequence_number: Option<String>,

        #[serde(
            rename = "subSequenceNumber",
            skip_serializing_if = "Option::is_none",
            default
        )]
        pub sub_sequence_number: Option<u64>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Record {
        #[serde(rename = "data")]
        pub base64_data: String,

        #[serde(rename = "partitionKey")]
        pub partition_key: String,

        #[serde(rename = "sequenceNumber")]
        pub sequence_number: String,

        #[serde(
            rename = "subSequenceNumber",
            skip_serializing_if = "Option::is_none",
            default
        )]
        pub sub_sequence_number: Option<u64>,

        #[serde(
            rename = "approximateArrivalTimestamp",
            skip_serializing_if = "Option::is_none",
            default
        )]
        pub approximate_arrival_timestamp_ms: Option<u64>,
    }

    impl Record {
        pub fn to_bytes(&self) -> Result<Vec<u8>, base64::DecodeError> {
            BASE64_STANDARD.decode(&self.base64_data)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LeaseLostMessage {}

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProcessRecordsMessage {
        #[serde(rename = "records")]
        pub records: Vec<Record>,

        #[serde(
            rename = "millisBehindLatest",
            skip_serializing_if = "Option::is_none",
            default
        )]
        pub millis_behind_latest: Option<u64>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ShardEndedMessage {}

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ShutdownMessage {
        #[serde(rename = "reason", skip_serializing_if = "Option::is_none", default)]
        pub reason: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ShutdownRequestedMessage {}
}
