# Research 428: Copilot ACP Host-Login Permission Proof Stop

Date: 2026-10-09

## Scope

Record the single bounded original-artifact attempt permitted first for exact
Darwin ARM64 Copilot CLI `1.0.93`, reusing the frozen artifact identity from
[Research 404](./404-copilot-cli-acp-offline-authentication-stop.md), fake
preparation from [Research 425](./425-copilot-acp-authenticated-proof-preparation.md),
and pre-prompt work from [Research 426](./426-copilot-acp-pre-prompt-discovery-stop.md).
The attempt uses the operator-approved existing host-owned GitHub login and
Auto selection policy. It was intended to send at most one benign task-scratch
prompt, cancel a permission request, and verify no effect. It does not qualify
any route or alter a production claim.

## Approved boundaries and fake proof

The agent and harness do not extract, copy, log or persist tokens, invoke
credential extraction, log in, switch accounts, export auth state, relocate
the home directory or change persistent settings. The hash-verified vendor
CLI may read its existing auth state through normal macOS `securityd` access;
the host does not offer item-level mediation here, so no item-level filter or
credential broker is claimed. The account reference is operator-reported and
was not independently queried.

Auto is a provider-selected policy, not a fixed model guarantee. Record an
underlying model only if exposed; otherwise report it unobserved. Vendor-owned
selection and retry behavior is allowed inside one ACP prompt and the enforced
60-second process/network ceiling; internal request count is unobserved. The
harness has no retry, resend or fallback. Egress is default-deny except for
the reviewed destinations in the official
[Copilot allowlist reference](https://docs.github.com/en/copilot/reference/copilot-allowlist-reference);
the execution record omits telemetry destinations. A local CONNECT proxy
checks the destination and matching TLS SNI before dialing. An unexpected
destination fails the attempt without expanding the allowlist.
The policy permits `github.com`, `github.githubassets.com`,
`avatars.githubusercontent.com`, `api.github.com`, `default.exp-tas.com`,
`copilot-proxy.githubusercontent.com`, `origin-tracker.githubusercontent.com`,
and subdomains of `githubcopilot.com`.

The fake-only ACP agent passed its permission-cancel/no-effect and negative
controls under the fake sandbox profile. It exercised an allowed local proxy
path and rejected an unlisted destination, TLS SNI mismatch, direct egress,
home and repository reads, writes outside task scratch, shell execution, late
approval, and effect creation. Its permission request was cancelled, and its
owned process and proxy were stopped and joined, including the forced-stop
control. These controls did not prove the exact original sandbox profile: the
fake profile maps Python `sys.prefix` for executable and dynamic-library reads,
while the original profile adds no such mapped root.

The original profile also denied every host-home read (profile SHA-256
`63ab94b84b72b376f2e9093b8e94dec79a210b098a02a85d416852f1924c0334`). The
approved boundary denies unrelated home reads while admitting only
vendor-owned authentication metadata reads required by that boundary. No fake
control admitted or exercised such a vendor-owned read, so the selected
original read set was not validated before launch. This broader denial could
have blocked startup if the CLI needed an approved home metadata read; with
stderr discarded and vendor-binary startup unobserved, that remains a possible
cause, not an established explanation. The committed
[preflight record](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/permission-proof-preflight-record.json)
and [execution record](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/permission-proof-execution-record.json)
are secret-free. The execution record binds the exact harness digest.

## Original `1.0.93` result

The bounded invocation for the exact official Darwin ARM64 `1.0.93` binary was
issued once with `--model auto --acp --stdio`. Its frozen executable SHA-256 is
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`. A
secret-free record was fsynced before the launcher command. The `sandbox-exec`
process returned code 1 after 1.263 seconds, before ACP `initialize`; the
JSON-RPC stream failed. The record's start flags establish that the bounded
launcher command was issued, but do not establish that the vendor binary itself
reached startup. The harness discarded stderr, so the failure cause and target
binary startup status are unknown. No destination reached the egress proxy; no
ACP authentication request, session, prompt, permission event, model identity
or effect was observed. The process group and proxy threads were joined.

This consumed the one allowed `1.0.93` invocation, even though no prompt was
sent. The harness correctly stopped: `1.0.81` and `1.0.80` were not started.
The shared prompt allowance remains unused, but this does not authorize a
second `1.0.93` invocation or change its one-invocation cap. No original
permission, cancellation or no-effect behavior was observed. The attempt is a
finite evidence stop, not a positive permission proof, compatibility result,
or qualification change. Any renewed `1.0.93` invocation requires separate
authority. The harness now reads and validates the committed execution record
before artifact staging or any original start. It compares the record against
its pinned SHA-256 and refuses all versions marked consumed; a missing or
changed record also stops execution. Since this record marks `1.0.93` consumed,
the current harness cannot repeat that attempt. Separate authority and a
reviewed update to the guard and record identity are required for renewed
execution.

## Result

The approved host-login and Auto boundaries are explicit in Contract 023. Fake
permission-cancellation controls pass, but they do not prove the exact original
profile or its host-home read set. The only authorized original start ended
before ACP initialization. Preserve all existing route claims, including the
exact `1.0.80` control. Do not start older versions or repeat `1.0.93` under
this proof allowance.
