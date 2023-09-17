# aws-sqs-types

Amazon SQS Struct for Rust.

## Install

`cargo add aws_sqs_types`

## Usage

```rust
use aws_sqs_types::Event;

let data: Event = serde_json::from_value(event).unwrap_or_default();
```

