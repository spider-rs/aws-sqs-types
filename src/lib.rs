use bytes::Bytes;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
/// A message value. Dynamic data from the user.
pub struct MessageAttributeValue {
    #[serde(rename = "dataType")]
    /// The logical data type amazon supports.
    pub data_type: String,
    #[serde(rename = "stringValue")]
    /// UTF-8 unicode.
    pub string_value: String,
    #[serde(rename = "binaryValue")]
    /// Binary value compressed data.
    pub binary_value: Option<Bytes>,
    #[serde(rename = "stringListValues")]
    /// List of strings.
    pub string_list_values: Option<Vec<String>>,
    #[serde(rename = "binaryListValues")]
    /// List of binary values.
    pub binary_list_values: Option<Vec<Bytes>>,
}

#[derive(Deserialize, Debug, Default)]
/// the sqs message.
pub struct Record {
    #[serde(rename = "awsRegion")]
    /// the region of aws.
    pub aws_region: String,
    /// event attributes for lambda.
    pub attributes: Option<EventAttributes>,
    /// the payload data
    pub body: String,
    #[serde(rename = "md5OfMessageAttributes")]
    /// md5 of message attriutes.
    pub md5_of_message_attributes: String,
    #[serde(rename = "md5OfBody")]
    /// md5 of payload body.
    pub md5_of_body: String,
    #[serde(rename = "eventSourceARN")]
    /// the event source arn.
    pub event_source_arn: String,
    #[serde(rename = "eventSource")]
    /// event source.
    pub event_source: String,
    #[serde(rename = "messageAttributes")]
    /// message attributes for extra fields.
    pub message_attributes: Option<std::collections::HashMap<String, MessageAttributeValue>>,
    #[serde(rename = "messageId")]
    /// a unique id for the message.
    pub message_id: String,
    #[serde(rename = "receiptHandle")]
    /// receipt handle.
    pub receipt_handle: String,
}

#[derive(Deserialize, Default, Debug)]
/// Event meta data
pub struct EventAttributes {
    #[serde(rename = "SentTimestamp")]
    /// the sent time
    pub sent_timestamp: String,
    #[serde(rename = "SenderId")]
    /// the send id target
    pub sender_id: String,
    #[serde(rename = "ApproximateReceiveCount")]
    /// the approximate receive counter
    pub approximate_receive_count: String,
    #[serde(rename = "ApproximateFirstReceiveTimestamp")]
    /// the approximate receive time
    pub approximate_first_receive_timestamp: String,
}

#[derive(Deserialize, Default, Debug)]
/// An Amazon SQS message event.
pub struct Event {
    #[serde(rename = "Records")]
    /// the records batch
    pub records: Vec<Record>,
}
