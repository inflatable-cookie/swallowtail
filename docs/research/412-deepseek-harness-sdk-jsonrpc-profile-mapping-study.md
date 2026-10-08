# 412 DeepSeek Harness SDK JSON-RPC Profile Mapping Study

Status: evidence-only mapping study; no qualification change
Owner: Swallowtail worker
Date: 2026-10-08

## Question

Does the current npm `dsh` SDK profile map to the already-qualified
`deepseek-harness.jsonrpc` implementation and its `deepseek-harness.runtime-bin`
axis at `0.1.0rc6`, or does the new SDK profile need its own route and artifact
axes?

This study was authorized by Tom's 2026-10-08 decision
[`f3ca9685-76b4-4e89-88f3-45f82bf8b4b2`](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#deepseek-harness-artifactprofile-mapping-study)
and Contract 023. It supersedes the earlier qualification outcome for this
worker. It does not extend a claim, add a behavior milestone, change a route or
matrix, or authorize runtime work.

## Method and frozen identities

The official npm channel was re-probed before artifact selection. At that
observation, `@deepseek-ai/dsh` `latest` and `next` were both `0.2.0-rc.2`;
`alpha` was `0.2.1-alpha.1`. The selected tarball was downloaded and its
SHA-512 matched registry integrity:

`EAJ3gPNcVt/uv8X19PMm9NkVhWgT7xXNMk0UKCVm+IQ5rpSQOcsMUa0HWlnYYVybKMsccjcRB21vVVsaXQ6IdA==`

The target was published on 2026-09-29. The current PyPI
`deepseek-harness-runtime-bin` macOS arm64 wheel observed in the retained
probe was `0.1.5rc1`, uploaded 2026-09-10. A query for PyPI
`0.2.0rc2` had already returned 404; it was not repeated. The npm CLI and
PyPI runtime wheel are separate artifact identities. Their version strings,
product name, and JSON-RPC method names do not establish a source or executable
mapping.

The downloaded npm profile subset contains complete tarball trees for six
selected packages (80 member files total). The exact tarball integrities and
per-member byte counts and SHA-256 digests are in the [npm package inventory](./deepseek-harness-sdk-jsonrpc-profile-mapping-npm-inventory.tsv):

| Package | Version | Members | Registry SHA-512 integrity |
| --- | --- | ---: | --- |
| `@deepseek-ai/dsh` | `0.2.0-rc.2` | 20 | `EAJ3gPNcVt/uv8X19PMm9NkVhWgT7xXNMk0UKCVm+IQ5rpSQOcsMUa0HWlnYYVybKMsccjcRB21vVVsaXQ6IdA==` |
| `@deepseek-ai/dsh-app-boot` | `0.2.0-rc.2` | 27 | `WvgNhBHSj85Z7u9vC1oQr9C7yZQ/L/JS+eVK4bueWEkaoqdEna504V5bi7qtWXxAz5JkeVZTy1sX86p8XG7LJg==` |
| `@deepseek-ai/dsh-base` | `0.2.0-rc.2` | 8 | `AclHClefnPmUe2qoPrKMUvauA/KelYevYlxFdHLZGcPRqrDH+D21HyrwJmj4f7Iu5OO0IqfjB4zBOkwMp5h1ug==` |
| `@deepseek-ai/dsh-sdk-app` | `0.2.0-rc.2` | 8 | `RX2ugii2C1AwJk/n1ajqypelNubOxdMfTNsG+vwTBEtonmjiJ4ACoKLFV/P6+RtyEeKEcveIEO1EmI6yXEjeMw==` |
| `@deepseek-ai/dsh-sdk-jsonrpc-server` | `0.2.0-rc.2` | 8 | `lNg524lActTvh27Z+FDQOD3n43hzVXe/rN9+q3gGBuUeUkMv+7Hw5TFiDqE6H4VzqAUvuQp8B/0LkChtY2Fi4A==` |
| `@deepseek-ai/dsh-sdk-protocol` | `0.2.0-rc.2` | 9 | `hs4Kl2x17wAuhSMUWuNpeHvv1mOm+F+A2EQnYu1QmxVux50PR3xHthee7IX/PylpgZybcfBn1JAigKvOFs5GPA==` |

The corresponding upstream source tag is `dsh-v0.2.0-rc.2`, resolving to
commit `639ed015397290b3745d163aafe02ffee4aa3f84`. Selected source-file
hashes are retained in the [tagged source file inventory](./deepseek-harness-sdk-jsonrpc-profile-mapping-source-files.tsv).
The source archive container digest is omitted because it was not
independently reproducible.
The npm package manifests name this repository and matching package paths and
versions, but the published package metadata has no `gitHead`. The tag is
therefore a separately frozen source snapshot, not proof that the PyPI wheel's
embedded executable was built from that commit or from the npm tarballs.

The exact PyPI runtime-bin artifact is
`deepseek-harness-runtime-bin==0.1.5rc1`, macOS arm64, wheel SHA-256
`65f20c541a499f08c36ac3bfdafce350ca8c7cc23488c1faf4d9b9524b547156`,
75,545,685 bytes. Its `deepseek-harness-sdk-runtime-macos-arm64` payload is
267,562,544 bytes, SHA-256
`6f98dfe1745f6c952c6157dfade5ac2c6a8aeb672291b2c0703b21adf11cf88c`.
The package marker reports `0.0.0-dev`, not an upstream source or build
revision. Its complete 11-member inventory is included with the other
published macOS arm64 runtime-bin wheels in the [wheel inventory](./deepseek-harness-sdk-jsonrpc-profile-mapping-runtime-bin-inventory.tsv).

No vendor artifact was executed, installed, or authenticated. No provider,
catalogue, or session request was made.

## Published runtime-bin hops after the selected point

The retained PyPI release query listed the following published points, in
order, after the existing `0.1.0rc6` selection. The complete inventories cover
all 64 member files across these six macOS arm64 wheels. The [hop ledger](./deepseek-harness-sdk-jsonrpc-profile-mapping-runtime-bin-hop-ledger.tsv)
records every added, removed, or changed member with both adjacent wheel
digests and classifies it. Distribution metadata and license changes are
retained there as well.

| Published point | Wheel SHA-256 | Selected-path classification |
| --- | --- | --- |
| `0.1.0rc7` | `39ae51dec905c496ede69250d51b02a424a6c96829feb2ad7e01b3cec97a3255` | The selected `dsh-jsonrpc-agent-pkg-macos-arm64` and its spawn helper changed digest. Both are opaque native payloads with no correlated source; behavior is unclassified. |
| `0.1.1rc1` | `2707cd666ba49ee0963228873abf7850ca7ec5e782cca61e3603793bace0d1cf` | The Python launcher and selected executable changed; an `-rg` sidecar appeared. The executable and utility remain opaque. |
| `0.1.2a3` | `fb82bf60c4cd5e793bed6caa5060589a31f349342bd9f4bb70826a854419c32d` | Structural runtime reset: old executable, old runtime `cordis.yml`, and old-named sidecars disappeared; `deepseek-harness-sdk-runtime-macos-arm64` and SDK-named sidecars appeared. The Python launcher also changed. This alpha point is not a substitute for the current npm target. |
| `0.1.2rc1` | `2cac2256cdebfda726c3378e24f4be31c0847aece7f2a7eb5a8523559f6b5894` | SDK runtime executable digest changed; its launcher and sidecars retained their preceding identities. Executable behavior is opaque. |
| `0.1.5rc1` | `65f20c541a499f08c36ac3bfdafce350ca8c7cc23488c1faf4d9b9524b547156` | SDK runtime executable and Python launcher changed; the `-rg` and spawn-helper sidecars retained their preceding identities. Executable behavior is opaque. |

Unchanged files and every wheel member digest are available in the inventory.
The PyPI chain establishes artifact succession and the package-level reset;
it does not establish source-level compatibility for opaque binaries. No
`0.2.0rc2` runtime-bin artifact was found. The alpha `0.1.2a3`, missing PyPI
target, and unclassified changed payloads remain holes.

## Selection and runtime control flow

The wheel's exact Python wrapper requires a non-empty `DSH_HOME`, selects the
platform's `deepseek-harness-sdk-runtime-*` executable by default, and on
POSIX replaces itself with that executable using the inherited environment
and unchanged command arguments. It does not inject a profile name. The wheel
README describes the carrier as the normal `dsh` CLI, but the wheel contains
no separate JSON-RPC executable and no source/build commit for its large
embedded runtime. Its embedded marker is only `0.0.0-dev`.

The npm CLI artifact is a profile launcher, not a platform runtime. Its shipped
`lib/bin.js` resolves `dsh --profile sdk`; the shorthand `dsh sdk` is expanded
to the same profile form. The profile loader layers bundle patches, profile
configuration, `$DSH_HOME/cordis.patch.yml`, and `--patch` overlays. The exact
SDK app patch disables HMR and inserts
`sdk-app-startup` and `sdk-jsonrpc-server`; the startup plugin claims stdio
only after accepting the profile command. These behaviors are present in the
published npm JS and YAML artifacts, regardless of the separately tagged
source snapshot.

This proves the selected npm profile's control path. It does not prove that
PyPI `0.1.5rc1` embeds the npm `0.2.0-rc.2` CLI, server, profile, or dependency
closure. The wheel's own runtime has no exposed dependency manifest connecting
its native payload to those npm packages.

## Operation, lifecycle, and authority mapping

| Boundary | Existing `0.1.0rc6` selected route | npm SDK profile `0.2.0-rc.2` static behavior |
| --- | --- | --- |
| Initialize | The prepared adapter selects one host-approved Cordis composition and supplies explicit provider, model, and cwd to the selected process. The exact `serverInfo.version` is wire metadata. | `initialize` resolves cwd and stores provider/model process-wide; optional `reasoningEffort` and `maxTokens` also apply to every SDK-created session. If no adapter is registered for the requested provider, only `deepseek-official` is dynamically mounted as a fallback; other unregistered providers fail. The handler resolves the provider/model call configuration before accepting the handshake. |
| Prompt and identity | Swallowtail runs one bounded structured prompt on its owned process. `{messageId}` is enqueue acknowledgement; the driver folds the selected turn through idle. | `session/prompt` creates or reuses an agent for each caller `sessionId`, queues content, and returns a durable `messageId`. The server can accept multiple session IDs and multiple prompts; a receipt does not identify a completed turn. The server also admits image blocks through the configured attachment store. |
| Events and idle | The existing decoder retains its exact `0.1.0rc6` handling of `session.event` and `session.status`; unknown namespaced observations stay bounded. | The server forwards session events and agent status notifications and has subagent lifecycle notifications. The protocol has no per-prompt result; clients own the idle/completion interval. Similar notification names do not prove equivalent state ownership or cardinality. |
| Shutdown | The selected driver sends `shutdown`, then joins process and task cleanup; cancellation force-stops the owned process. It has no native cancel method. | The server shutdown disposes all SDK-created agents, the optional fallback adapter, and subscriptions. The plugin flushes the response, disposes the root runtime, then exits. Stdin EOF and SIGINT/SIGTERM have separate profile-launcher shutdown paths. The JSON-RPC method set has no cancel or per-session close. No process was run, so timeout, signal, and join behavior are not black-box qualified. |
| Persistence | The Swallowtail route claims no reusable continuation binding; host configuration remains host-owned. | The base composition configures durable JSONL sessions under `$DSH_HOME/sessions` and a session projection cache; the server creates or reuses agents by session ID. Restart restoration was not exercised. This changes the declared lifecycle and state ownership. |
| Host, cwd, and environment | A host-approved executable and Cordis `EnvironmentRef` are bound into preparation. The adapter opens no credential lease. Cordis/provider/tool authority remains host-owned; no local-server evidence transfers. | The Python wrapper inherits the caller's environment and requires `DSH_HOME`; the SDK profile loader also admits profile, home, and launch overlays. Its base patch config reads inherited environment, `$DSH_HOME/.credentials.yaml`, and project/user `.env` fallbacks. The SDK server can mount a `deepseek-official` provider adapter. This is a different credential and configuration path. |
| Tools and permissions | The prepared route records a host-selected access profile and does not advertise a consumer permission exchange or tool callback. | The SDK base profile defaults to `read`, `write`, and `edit`; its composition also includes shell and other bundled services. The base patch defaults `DSH_PERMISSION_MODE` to `workspace-write` with approval policy `ask`; an explicit `danger-full-access` setting switches the policy to `never`. This is application policy on the host process, not evidence of a new OS isolation boundary. |

The method names and enqueue receipt overlap, but the newer profile adds
persistent multi-session lifecycle, process-wide dynamic provider selection,
local credential fallbacks, profile overlays, image input, and a permissioned
tool composition. These differences are visible in the exact selected npm
server JS and SDK/base package trees. The wheel's runtime implementation
remains opaque and uncorrelated to that npm package set.

## Mapping result and next adaptation

The evidence does not establish a same-contract extension on
`deepseek-harness.runtime-bin`. In particular, the `0.1.2a3` wheel removes the
old selected executable and Cordis file, while the latest `0.1.5rc1` wheel
contains a differently named, much larger SDK runtime. The current npm SDK
profile independently exposes state, credential, and permission behavior that
the current one-shot route does not claim. No private behavior milestone can
make these package identities or lifecycle boundaries equivalent.

For a separate ruling, propose a distinct SDK route, provisionally
`deepseek-harness.sdk-jsonrpc`, with independently pinned artifacts:

- a candidate runtime carrier axis `deepseek-harness.sdk-runtime-bin`, beginning
  with the complete published ledger retained, including the `0.1.2a3` alpha
  as an unqualified hole; the current observed wheel is exact `0.1.5rc1`. A
  separate ruling must name the first claimed point and any maintained segment;
- a separate SDK profile axis for exact CLI, SDK app, JSON-RPC server/protocol,
  and base-profile package identities. The observed npm point is exact
  `0.2.0-rc.2` for the six packages inventoried above;
- no claim between these axes until the vendor supplies exact build/source
  provenance (or an equivalent immutable dependency manifest) tying the
  `0.1.5rc1` native runtime to a complete profile/server dependency closure;
- a separately authorized provider-free offline behavior proof that exercises
  profile selection, initialize, queued prompt, idle observation, persistent
  sessions, graceful shutdown, signal/forced cancellation, process join,
  credential source boundaries, and tool/permission effects against fakes.

The route and axis labels above are a proposal, not production claims. The
study does not qualify `0.1.5rc1` or `0.2.0-rc.2`, change any existing claim
or exclusion, grant artifact-execution authority, or transfer evidence to the
DeepSeek local Web `/api` route. Keep the current `0.1.0rc6` point and all
existing route, axis, baseline, and exclusion records intact until a separate
ruling and qualification task.
