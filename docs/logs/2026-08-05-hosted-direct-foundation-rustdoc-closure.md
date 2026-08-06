# 2026-08-05 Hosted Direct Foundation Rustdoc Closure

## Result

The first provider-adapter Rustdoc batch closes
`swallowtail-adapter-anthropic`, `swallowtail-adapter-deepseek`,
`swallowtail-adapter-kimi-platform`, and `swallowtail-adapter-xai` under
crate-root denied missing docs.

The review keeps their routes distinct. Anthropic Messages exposes catalogue,
one-attempt inference, and consumer-owned tool continuation. Anthropic Managed
Agents separately exposes durable provider resources, authoritative-history
reattachment, read-only reconciliation, and recovered-resource cleanup.
DeepSeek keeps private reasoning continuation and explicit unmanaged-cache
acceptance. Kimi Platform keeps catalogue and one-attempt K3 inference. xAI
keeps its read-only Models route separate from serial Responses WebSocket
sessions and runs.

No preparation object gains routing, credentials, provider effects, retry, or
fallback authority. Low-level drivers remain public escape hatches. The batch
removes 308 warnings, reducing the workspace from 2,191 to 1,883 without
suppression. Eleven of 27 packages now enforce denied missing docs; the
remaining warnings belong to 16 provider adapters.

## Validation

- focused validation passed 127 tests across the four adapter packages
- warnings-denied clippy passed for all four packages
- extracted package proof passed for all four archives
- crate-root denied-missing-doc Rustdoc passed for all four packages
- workspace all-feature Rustdoc completed with 1,883 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with the broader hosted packages: Alibaba Model Studio,
Bedrock, and OpenAI. Then close the remaining installed-harness and local-model
adapters in package-sized groups.
