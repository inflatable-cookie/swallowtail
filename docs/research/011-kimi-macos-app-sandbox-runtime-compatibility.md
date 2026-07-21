# 011 Kimi macOS App Sandbox Runtime Compatibility

Status: promoted
Owner: Tom
Updated: 2026-07-21

## Question

Can the exact Kimi Code `0.28.1` arm64 artifact run as a supported inherited
macOS App Sandbox helper while one user-selected project grant reaches its
shell and background descendants?

## Method

The proof used the official artifact pinned by Research 010 on macOS `26.5.2`
arm64. A temporary native app presented `NSOpenPanel`, created a persisted
security-scoped bookmark, converted the selected root to a regular
interprocess bookmark, and passed it to an embedded launcher. The launcher
resolved the bookmark, began scoped access, changed to the selected root, and
executed either a deterministic descendant probe or Kimi `--version`.

The app and helpers were ad-hoc signed with hardened runtime because this host
has no installed development or distribution identity. This proves local
development behavior only; it does not claim production signing,
notarization, or distribution readiness. No Kimi account, credential, login
mutation, provider network request, or inference occurred.

## Apple Authority

Apple's current helper guidance requires an embedded command-line helper to
carry only App Sandbox and sandbox inheritance entitlements. It warns that
adding other entitlements can cause launch or code-signing failures. Apple's
file-access guidance documents recursive user-selected folder grants,
persisted security-scoped bookmarks, and regular bookmark handoff to another
process that resolves the bookmark.

This rules out treating Kimi's upstream dynamic-code entitlements as an
undocumented exception to the inherited-helper model.

## Generic Boundary Result

The supported two-entitlement launcher received the selected root and passed
the dynamic extension through `exec` to a deterministic descendant. All twelve
bounded checks passed:

- direct read, write, and in-root rename succeeded
- shell, nested shell, and background-child work inside the root succeeded
- outside read, write, and rename failed
- shell access outside the root failed
- symlink escape failed
- outbound loopback network failed without a network entitlement

This proves the bookmark handoff and inherited filesystem boundary for a
compatible executable. It does not qualify a Kimi host service by itself.

## Exact Kimi Result

The upstream Kimi executable needs four dynamic-code entitlements: JIT,
unsigned executable memory, dyld environment variables, and disabled library
validation. Re-signing the exact binary with only Apple's required App Sandbox
and inheritance entitlements produced deployment digest
`57813cdfbb8aa81511139fb1c3fa3322ae98c7d114199ff4095e15b332562c5a`.
Kimi then terminated with signal 5 during V8 heap initialization.

A diagnostic re-sign that retained all four upstream dynamic-code entitlements
alongside App Sandbox and inheritance also terminated with signal 5 during V8
heap initialization. It is outside Apple's documented inherited-helper shape
and did not preserve runtime behavior anyway.

One final diagnostic used `NODE_OPTIONS=--jitless` with the documented helper
entitlements. It passed V8 initialization but stalled while loading Kimi's
extracted clipboard native module. That arm64 module has SHA-256
`dd89f4f323b6668cd3e28a0b34954b514f975db13953de52e38d3ad1f729e96a`,
an ad-hoc linker signature, and no Team ID. JITless is not a documented Kimi
deployment mode and does not solve the native-module signing boundary. The
hung child and its parent were terminated and observed before the experiment
ended.

## Decision

The native App Sandbox bookmark mechanism is viable for compatible helpers,
but the exact Kimi Code `0.28.1` single-executable artifact does not qualify as
an inherited helper. Contract 017's stop condition fires before portable
containment records or a host-enforced Kimi profile are added.

Research 012 and the operator's later decision make containment an optional,
explicit capability. This result therefore does not block Kimi ACP
communication. It proves that `HostEnforced` is unavailable for the pinned
artifact while an explicit `AmbientHost` driver may proceed without a bounded
filesystem or descendant claim. A custom source build, app-hosted runtime, XPC
design, container, VM, or later provider sandbox remains a separate configured
route with its own authority evidence.

## Promotion

- Contract 017 now requires exact executable and dynamic-module compatibility.
- Research 012 limits that qualification gate to enforced isolation claims.
- Research 009's mechanism selection is qualified by this failed artifact
  result.
- Research 010's artifact-runtime stop condition fired as designed.
- Card 057 records the unavailable host-enforced capability; roadmap 018
  continues through an explicit ambient driver lane.

## Primary Sources

- [Apple: Embedding a helper tool in a sandboxed app](https://developer.apple.com/documentation/xcode/embedding-a-helper-tool-in-a-sandboxed-app)
- [Apple: Accessing files from the macOS App Sandbox](https://developer.apple.com/documentation/security/accessing-files-from-the-macos-app-sandbox)
- [Kimi Code `0.28.1` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.28.1)
