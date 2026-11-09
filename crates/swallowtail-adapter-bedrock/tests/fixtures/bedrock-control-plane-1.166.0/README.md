# Bedrock catalogue SDK 1.166.0 identity

This frozen fixture records the official crates.io `aws-sdk-bedrock` artifacts
from `1.161.0` through `1.166.0`, their exact source identities, and the
complete extracted crate tree inventory. Every downloaded `.crate` digest
matched the crates.io versions API checksum.

The selected operation remains `ListFoundationModels`: unfiltered, paginated
once by the adapter, one provider request, one SDK attempt, explicit delegated
identity, and bounded observation of the catalogue summary fields. The fixture
contains no credentials, account ids, or host paths. No AWS calls or downloaded
artifact execution were performed.

The only generated-code changes are additive guardrail contextual-grounding
tier types with their re-exports (1.162.0) and a `#[cfg(test)]`-only endpoint
rule-set test-builder adjustment (1.166.0); the adapter maps no guardrail
type. These releases move the SDK's declared Smithy dependencies past the
compatibility line that carried `pin-utils`; the workspace lock drops the
`pin-utils` package when both Bedrock SDKs resolve together.
