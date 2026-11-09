# Bedrock Runtime SDK 1.153.0 identity

This frozen fixture records the official crates.io `aws-sdk-bedrockruntime`
artifacts from `1.148.0` through `1.153.0`, their exact source identities, and
the complete extracted crate tree inventory. Every downloaded `.crate` digest
matched the crates.io versions API checksum.

The selected operation remains `ConverseStream`: one user text message, one
maximum output-token setting, one SDK attempt, typed EventStream decoding,
bounded assistant-text and token-usage projection, and fail-closed handling of
unknown or unsupported event semantics. The fixture contains no credentials,
provider payloads, or host paths. No AWS calls or downloaded artifact execution
were performed.

No selected operation, event, usage, error, or shared configuration file
changed across these hops. The only generated-code changes are two unselected
InvokeModel serializer refactors (1.149.0) and a `#[cfg(test)]`-only endpoint
rule-set test-builder adjustment (1.153.0). These releases move the SDK's
declared Smithy dependencies past the compatibility line that carried
`pin-utils`; the workspace lock drops the `pin-utils` package when both Bedrock
SDKs resolve together.
