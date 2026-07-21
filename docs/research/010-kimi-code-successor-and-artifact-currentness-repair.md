# 010 Kimi Code Successor And Artifact Currentness Repair

Status: promoted
Owner: Tom
Updated: 2026-07-21

Research 011 records the resulting native proof. The exact pinned artifact
does not retain runtime behavior under the documented App Sandbox inherited-
helper signature, so the stop condition below has fired.

## Question

Does the existing `0.28.1` corpus already target the maintained Kimi Code
successor, and which exact executable, state, signing, ACP, and upgrade inputs
must precede the native macOS containment proof?

## Method

Official Moonshot AI releases, the annotated `0.28.1` tag, its peeled source
commit, tagged package manifests and lockfile, current Kimi documentation,
upstream ACP releases, and the official macOS arm64 archive were checked on
2026-07-21.

The archive was downloaded to a temporary directory only. Its digest, archive
layout, Mach-O identity, signature, and entitlements were inspected. The Kimi
binary was not launched. No account, login state, credential store, model
configuration, provider request, or inference route was inspected.

## Currentness Repair

The existing corpus already targets the maintained TypeScript Kimi Code
successor. Research 009 incorrectly described that corpus as the wound-down
Python line and treated the older documentation entry `0.26.0` as a replacement
for `0.28.1`.

Official release evidence instead shows:

- repository: `MoonshotAI/kimi-code`
- release: `@moonshot-ai/kimi-code@0.28.1`
- published: `2026-07-20T15:00:02Z`
- annotated tag object: `0032545b65f95c139ecba5a48ba1b911844e1ffe`
- peeled source commit: `efacf0452d46f5dbd67499eabc053869495d5213`
- package: `@moonshot-ai/kimi-code` `0.28.1`
- command entry: `kimi`; ACP argument: `acp`

The official documentation changelog still stops at `0.26.0`. It lags the
provider release record and cannot demote a newer official tag. Release and
tagged-source evidence govern the exact artifact pin; current documentation
still supplies supported behavior and configuration guidance where it agrees
with tagged source.

Research 006 also called annotated tag-object ids “tag commits.” The underlying
behavioral pin was correct, but artifact identity is now split precisely:

| Artifact | Tag object | Peeled source commit |
| --- | --- | --- |
| Kimi Code `0.28.1` | `0032545b65f95c139ecba5a48ba1b911844e1ffe` | `efacf0452d46f5dbd67499eabc053869495d5213` |
| ACP `schema-v1.19.1` | `0331ddaea7814afa147194cd5e93495a2a0f8c82` | `d0549a115750a0a25c1c5631dd72e0d248859aa4` |

## ACP Comparison

The `0.28.1` successor retains every selected corpus behavior. No shared ACP
framing or Contract 017 change is needed.

| Behavior | Current tagged evidence | Fixture decision |
| --- | --- | --- |
| transport | newline-delimited JSON-RPC on stdin/stdout; logs on stderr; no banner | retain |
| initialize | wire `1`; image and embedded-context prompt evidence; HTTP/SSE MCP evidence | observe, do not claim richer inputs or MCP |
| new | creates provider-owned session | retain only as binding source |
| load | resumes stored state, awaits ordered replay, then responds | retain |
| resume | separately advertised and implemented without replay | retain |
| prompt | text prompt and ordered updates | retain bounded text subset |
| writes | `fs/write_text_file` reverse RPC when advertised | retain bounded callback subset |
| permission | `session/request_permission` reverse RPC | observe, never approve |
| cancellation | `session/cancel` terminates the active prompt | retain |
| disconnect | EOF or signal closes the harness once | retain joined owner cleanup |
| close | no stable `session/close` or logout | retain connection-owned close |
| shell | executes locally, not through ACP terminal reverse RPC | exclude; containment still required |

Kimi still does not enforce the load or resume request `cwd`; stored provider
state owns its work directory. Arbitrary session ids remain unauthorized. The
durable binding and fail-closed resource checks in Contract 017 remain required.

The exact Kimi package still contains `@moonshot-ai/acp-adapter` `0.3.4` and
locks `@agentclientprotocol/sdk` `0.23.0`. The current stable schema remains
`schema-v1.19.1`; wire version remains integer `1`. Agent, adapter, SDK, schema,
and wire identities stay independent.

## Executable And Signing Input

The first containment proof targets the current `darwin-arm64` host and exact
official release asset:

| Input | Exact evidence |
| --- | --- |
| archive | `kimi-code-darwin-arm64.zip`, 50,630,141 bytes |
| archive SHA-256 | `fa93e9daa30449c5cb32d8adb2a75651ec6c60dcd72fd4bf65c530edb8c144f9` |
| archive layout | one root member named `kimi` |
| executable | arm64 Mach-O, 158,004,544 bytes |
| executable SHA-256 | `297d4521d23ebab163fd778dfc3422dcb2cc216a6a0eadd535b914f20c538cd6` |
| upstream identifier | `kimi` |
| upstream team | `2J9472RW75` |
| upstream authority | `Developer ID Application: Beijing Moonshot Technology Co., Ltd (2J9472RW75)` |
| upstream signature | valid on disk, hardened runtime |

The upstream executable carries dynamic-code entitlements for JIT, unsigned
executable memory, dyld environment variables, and disabled library
validation. It does not carry App Sandbox or inheritance entitlements. The
unmodified provider signature therefore cannot be the final sandbox helper
signature.

Card 057 must treat the verified archive and executable as upstream inputs,
then produce a deployment-signed embedded helper with App Sandbox and
inheritance entitlements. Re-signing changes the executable digest. The proof
must preserve the upstream digest as provenance, record the signed output
digest separately, and prove the Node single-executable runtime still works
under the combined entitlement boundary. Failure to preserve runtime behavior
or descendant containment is a stop condition, not permission to weaken the
sandbox.

Notarization, distribution identity, and consumer bundle policy remain
consumer deployment authority. The first proof does not need a production
signing identity.

## State And Upgrade Gate

The selected process environment requires:

- `KIMI_CODE_HOME` resolved from one host-approved isolated provider-state root
- no ambient home-directory or provider-state fallback
- `KIMI_CODE_NO_AUTO_UPDATE=1`

Tagged source documents and tests confirm that the last variable disables the
update check, background installation, and prompt. The legacy alias is not
needed in the supported manifest.

The host resolves one pinned embedded executable rather than searching
`PATH`. Before launch it matches the configured tag, source commit, upstream
archive digest, and deployment-signed helper digest. Initialization then
matches agent version, wire, and the selected capability matrix before any
session work. Floating `latest`, `kimi upgrade`, provider self-update, digest
drift, and version drift fail closed.

## Access And Capability Boundary

The seamless first route uses pre-existing delegated Kimi Code login state in
the isolated root. Advertised terminal login remains evidence only until a
separate explicit operator-authorized action exists. Swallowtail never reads or
exports the credential.

Membership OAuth, membership-backed API keys, Kimi Platform API keys, and
configured external-provider credentials remain separate configured instances.
The selected fixture does not substitute among them.

Advertised image, embedded-context, HTTP/SSE MCP, session-list, mode/config,
and unstable model-selection capability remains excluded. Local shell,
plugins, background work, and subagents also remain excluded. Their presence
in the successor strengthens the containment requirement; it does not enlarge
the first adapter subset.

## Promotion

- Research 009's native macOS mechanism selection remains active.
- Research 009's Kimi successor/version delta is superseded by this record.
- Research 006 remains the behavioral basis with corrected tag identities.
- Contracts 015 and 017 remain unchanged.
- The corrected deterministic corpus and upgrade gate live in card 065.
- Card 057 may now bind the exact arm64 upstream and deployment-signing inputs.

## Primary Sources

- [Kimi Code `0.28.1` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.28.1)
- [Kimi Code tagged source](https://github.com/MoonshotAI/kimi-code/tree/%40moonshot-ai%2Fkimi-code%400.28.1)
- [Kimi Code ACP reference](https://www.kimi.com/code/docs/en/kimi-code-cli/reference/kimi-acp)
- [Kimi Code environment variables](https://www.kimi.com/code/docs/en/kimi-code-cli/configuration/env-vars.html)
- [Kimi Code changelog](https://www.kimi.com/code/docs/en/kimi-code-cli/release-notes/changelog.html)
- [ACP schema `v1.19.1`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.19.1)
- [Apple: Embedding a command-line tool in a sandboxed app](https://developer.apple.com/documentation/xcode/embedding-a-helper-tool-in-a-sandboxed-app)
