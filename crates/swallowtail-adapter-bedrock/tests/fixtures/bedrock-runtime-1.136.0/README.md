# Bedrock Runtime SDK Fixture

This corpus pins the official `aws-sdk-bedrockruntime` crate at `1.136.0` and
constructs its generated `ConverseStream` event and error types directly.

It covers the first text-only, one-attempt subset from Contract 019. It does
not contact AWS, resolve credentials, read AWS configuration, invoke a model,
or exercise the separate Bedrock control-plane catalogue.
