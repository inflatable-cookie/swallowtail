# 223: Qualify corrected Bedrock SDK graph and remove pin-utils

### Compatibility

- qualify `bedrock.runtime` through official crates.io stable
  `aws-sdk-bedrockruntime 1.153.0` and `bedrock.catalogue` through
  `aws-sdk-bedrock 1.166.0` as compatible extensions, walking the five
  published stable hops `1.149.0`–`1.153.0` and `1.162.0`–`1.166.0` after the
  Research 381/386 ceilings. Both SDK crates ride one
  awslabs/aws-sdk-rust release train; the coordinated move crosses the Smithy
  compatibility line (`aws-smithy-schema` `0.2.x` → `1.x`,
  `aws-smithy-json` `0.63.x` → `0.65.x`) that removes `pin-utils 0.1.0`
  (RUSTSEC-2026-0441) from the resolved workspace lock, and moves the
  `aws-smithy-http-client` test fake exact pin to `1.5.2`. The maintained
  segments extend to `1.139.0..=1.153.0` (Runtime) and `1.157.0..=1.166.0`
  (catalogue) on the existing claim ids and behavior revisions; exact
  `1.136.0` and `1.148.0`, the `1.137.0`/`1.138.0`/`1.149.0` holes, and
  yanked `1.144.0`/`1.156.0` stay excluded, and later stables remain
  `UnverifiedNewer`. Selected ConverseStream and ListFoundationModels
  surfaces stay byte-identical with frozen complete tree inventories,
  per-hop classifications, and real-SDK fake-transport wire proofs; exact
  service revisions, access profiles, endpoint audiences, one-attempt retry,
  and credential custody are unchanged. `deny.toml` advisory ignores stay
  empty and `effigy security:dependencies` passes. No operation,
  capability, authority, or lifecycle change. Research 381, 386, 476.
