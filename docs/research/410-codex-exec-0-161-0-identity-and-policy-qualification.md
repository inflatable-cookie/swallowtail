# Codex exec 0.161.0 identity and policy qualification

Status: provider-free adaptation and compatible extension completed.
Date: 2026-10-08.
Scope: `codex.exec` on `codex.cli` only.

## Decision

Extend `codex.exec.cli-window-2` through official stable `0.161.0` on the
existing maintained `codex.exec.jsonl-v1` behavior. Preserve the `0.80.0`
baseline, previous segments and exclusions, and the independent app-server
claim. Keep the unpublished `0.155.2` point as an unsupported gap between the
two maintained exec segments. No public operation, lifecycle, or consumer
contract changed.

The projectless trust behavior is compatible with this route. Codex may skip
automatic persisted project trust when exec starts with an explicit projectless
cwd. The adapter continues to pass the caller's exact working-resource
reference and does not recreate persisted trust or add trust-related flags.

System and macOS MDM `application.network` allowlists remain authoritative for
Codex network requests, including regular exec. Host approval and the existing
`web_search="live"` configuration do not bypass them. Requirements refresh
remains bound to the active policy; refresh failure or an unavailable policy
does not become an allow decision. Swallowtail does not set
`ignore_managed_requirements`, broaden network access, or claim process
containment from this application-level policy.

## Official identity and frozen evidence

The npm `@openai/codex` `latest` channel and GitHub latest stable channel agree
on `0.161.0` / `rust-v0.161.0`; the latest prerelease is not a stable route
point. Research 370 freezes the complete official stable sequence after
`0.155.1`:

`0.156.0`, `0.156.1`, `0.157.0`, `0.157.1`, `0.158.0`, `0.159.0`,
`0.159.1`, `0.159.2`, `0.159.3`, `0.160.0`, `0.160.1`, `0.161.0`.

The exact tag for `0.161.0` resolves to source commit
`979011409de0a60b52f179721948e65531d26144`. Research 370 freezes the full
published-hop source name/status ledger, tag objects and commits, selected
source classifications, wrapper and selected native package archives, and
complete per-file inventories. Its identity and package/platform chain is
corroborated by Research 388, which freezes all six platform package variants
for each of the thirteen stable points and their complete source inventories.
This task independently recomputed the archive SHA-256, npm SRI, extracted
paths, per-file hashes, file counts, and unpacked sizes for all 39 wrapper,
Darwin arm64, and Linux x64 artifacts in the Research 370 corpus. None was
executed.

The adapter identity regression pins SHA-256 for Research 370's report,
identity, artifact ledger, complete artifact-file inventory, source tags,
complete source hop ledger, and selected source map. It also asserts the exact
selected path sets for both original stops and the `0.160.1` → `0.161.0`
network-policy refresh. This makes the reused classification corpus sensitive
to changes in either content or membership.

## Published-hop findings

| Hop | Selected `codex.exec` result |
| --- | --- |
| `0.155.1` → `0.156.0` | Accept the projectless startup change without restoring trust persistence. Keep query progress and ignore additive search result fields. |
| `0.156.0` → `0.156.1` | No additional selected exec implementation change. |
| `0.156.1` → `0.157.0` | Accept system and macOS MDM `application.network` destination policy for ordinary exec. |
| `0.157.0` → `0.157.1` | No selected exec implementation change. |
| `0.157.1` → `0.158.0` | The exec source delta is test-only for the selected behavior. |
| `0.158.0` → `0.159.0` | Internal app-server runtime path options change; external argv and selected JSONL framing remain unchanged. |
| `0.159.0` → `0.159.1` | No changed selected exec implementation. |
| `0.159.1` → `0.159.2` | No changed selected exec implementation. |
| `0.159.2` → `0.159.3` | No changed selected exec implementation. |
| `0.159.3` → `0.160.0` | No changed selected exec implementation; the config helper leaves the prior fallback unchanged. |
| `0.160.0` → `0.160.1` | No changed selected exec implementation. |
| `0.160.1` → `0.161.0` | Managed-policy refresh keeps the active policy bound. The adapter adds no override; optional Daybreak/Cyber code is not selected. |

The complete upstream path classifications remain in the digest-pinned
Research 370 source ledger. Relevant stop regressions identify the projectless
classification and thread-start files, the managed-requirements loader and
application-network policy files, and their upstream tests. The final refresh
hop's selected files are exactly `application_network.rs`,
`in_process_bootstrap.rs`, and `in_process_bootstrap_tests.rs` under
`codex-rs/app-server/src/`. Changed SDK, UI, unrelated provider, and optional
access-program paths stay bounded as unselected by that corpus.

## Adapter evidence and boundaries

The selected prepared-exec path is exercised with fakes at `0.161.0`:

- explicit projectless and marked working-resource references reach the
  process request unchanged;
- invocation retains JSONL, ephemeral execution, read-only sandbox, disabled
  approval, user-config and rule suppression, and an exact saved-login
  environment; no managed-requirement override or trust bypass is passed;
- host-approved search dispatch retains `web_search="live"`, projects only
  query progress, and ignores additive result details;
- a provider denial after policy refresh becomes the fixed
  `swallowtail.codex.exec.provider_failed` diagnostic with private provider
  text omitted;
- cancellation still yields `Cancelled`, force-stops the owned process, and
  joins cleanup.

These fakes prove the adapter's actual selected path and projection. They do
not emulate macOS MDM or make a live network-allowance claim. The managed
refresh and fail-closed behavior is established by frozen official source and
upstream regression files; allowed destinations remain subject to the active
host policy.

## Claim shape and preserved boundaries

The maintained claim remains `codex.exec.cli-window-2` with
`AllowUnverified`, baseline `0.80.0`, and the existing exclusions
`0.108.0`, `0.109.0`, `0.149.2`, `0.150.2`, `0.151.1`, `0.152.2`, and
`0.154.1`. The existing `0.122.0` → `0.155.1` segment remains intact. A
second `0.156.0` → `0.161.0` segment carries the same maintained status and
behavior revision; the missing `0.155.2` point remains incompatible. Exact
stable points through `0.161.0` are qualified, and later stable points remain
visible as `UnverifiedNewer`. The additive search-results mapping, invocation
argv, failures, access limits, cancellation, and joined cleanup remain
unchanged.

The app-server claim and lifecycle are separate and unchanged by this exec
decision. This qualification does not prove provider sessions, login, model
catalogue behavior, live search availability, operating-system process
containment, or any provider/host mutation.

## References

- [Research 370 Codex exec currentness stop](./370-codex-exec-currentness-stop.md)
- [Research 388 Codex app-server identity](./388-codex-app-server-currentness-qualification.md)
- [Codex `0.161.0` source tag](https://github.com/openai/codex/tree/rust-v0.161.0)
- [Official npm package](https://www.npmjs.com/package/@openai/codex)
- [Official GitHub releases](https://github.com/openai/codex/releases)
