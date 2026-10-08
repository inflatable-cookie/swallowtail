# Bedrock catalogue SDK 1.161.0 identity

These fixtures freeze the official `aws-sdk-bedrock` crates.io stable release set from the workspace’s 1.150.0 pin through 1.161.0. `identity.json` records exact crate archive checksums, packaged source commits, release metadata, and declared runtime dependency requirements. `dist-inventory.json` contains complete 1,470-file trees and classified changes for every published hop. `protocol.json` freezes the selected ListFoundationModels request, typed response projection, failure, access, retry, and route boundaries.

The adapter’s opt-in telemetry remains off, its request is unfiltered and non-paginated, and its one-attempt configuration prevents retry-classifier additions from creating another provider request. The yanked 1.156.0 point and independently unqualified 1.149.0 point remain excluded. Other generated Bedrock operations and types stay unmapped.

No crate artifact was executed. No AWS request, live catalogue, credentials, host SDK installation, or host update was used.
