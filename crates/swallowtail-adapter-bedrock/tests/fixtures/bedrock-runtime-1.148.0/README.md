# Bedrock Runtime SDK 1.148.0 identity

This frozen fixture records the official crates.io `aws-sdk-bedrockruntime`
artifacts from `1.139.0` through `1.148.0`, their exact source identities, and
the complete extracted crate tree inventory. Every downloaded `.crate` digest
matched the crates.io versions API checksum.

The selected operation remains `ConverseStream`: one user text message, one
maximum output-token setting, one SDK attempt, typed EventStream decoding,
bounded assistant-text and token-usage projection, and fail-closed handling of
unknown or unsupported event semantics. The fixture contains no credentials,
provider payloads, or host paths. No AWS calls or downloaded artifact execution
were performed.

The crate tree shows selected-operation changes only in telemetry/retry and
response-deserializer plumbing, plus optional request-member serialization
that the adapter does not set. No selected event, usage, or generated error
type files changed. The `1.144.0` artifact is yanked and remains excluded.
