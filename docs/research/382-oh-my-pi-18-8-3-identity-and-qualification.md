# 382 Oh My Pi 18.8.3 Identity and RPC Qualification

**Scope:** `oh-my-pi.rpc` only. This record qualifies the current official
`@oh-my-pi/pi-coding-agent` stable after the `18.2.7` ceiling. Pi RPC and the
Pi SDK sidecar retain independent axes and evidence.

## Official identity

At the 2026-10-08 observation, npm `latest` and GitHub's latest stable release
agreed on `18.8.3`. npm published it at `2026-10-07T19:08:36.958Z`; GitHub's
latest tag is `v18.8.3`, commit
[`3e3c488a58d294e3a10051da588628e2cfb9d35c`](https://github.com/can1357/oh-my-pi/commit/3e3c488a58d294e3a10051da588628e2cfb9d35c).
The registry package has `gitHead: null`, so the verified published npm tree
is the artifact authority. The tarball SHA-256 is
`90ee818ddd5000405446e2396d4d7baeadabf04aff141fc311d47c99f7de17d2`, npm
integrity is
`sha512-v36DfSnLgOQkoglFIh9/5GCZPAwVLD0WHfMEoIf3jE6gKyXC30DJdGPTI7aNx+rqVRS9lkxDc/nUoqyi1qmZug==`,
and `dist/cli.js` is SHA-256
`ab1482816804bc7b73d1b1407f62e06656657f9d9636e860466ab004bd20cef2` at
29,155,193 bytes. The package has 3,179 files and requires Bun `>=1.3.14`.
The read-only host observation was `omp 18.3.4`, arm64, and Bun `1.4.2`; no
artifact was run, installed, or used to contact a provider.

The identity freeze is
[`identity.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.8.3/identity.json).
It records exact npm and GitHub channel metadata, all published artifact
digests, source/tag identity, runtime observations, and the previous claim.

## Every published hop

The complete inventory begins at `18.2.7` and covers all 32 published npm
stable hops through `18.8.3`, including npm-only `18.2.9`. It contains 33
complete path-to-SHA-256 trees. The `18.2.7` tree has 2,700 files; `18.8.3`
has 3,179 files and tree digest
`05d484a933e70894931f66fec4a9ef6bcf493584872f83e230063f011d85c551`.
The per-hop ledger recomputes every package added, removed, or changed path,
and classifies 360 changed selected-source records across 89 exact source
paths. The 473 selected source/version snapshots match their exact npm tree
digests; there are no source mismatches or unmatched selected paths. The
full fixtures are
[`complete-tree-inventory.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.8.3/complete-tree-inventory.json),
[`hop-ledger.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.8.3/hop-ledger.json),
and [`protocol.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.8.3/protocol.json).

| Published hop | Selected-surface classification | Finding |
| --- | --- | --- |
| `18.2.7→18.2.8` | compatible internal | Browser/transcription additions and internal URL refactor; selected commands, schemas, and frames stay. |
| `18.2.8→18.2.9` | compatible internal | Model, read/grep, and session internals change; exact selected contract stays. npm tree is authoritative; no GitHub tag exists. |
| `18.2.9→18.2.10` | compatible tool internal | Grep/glob and session internals change; selected names and input keys stay. |
| `18.2.10→18.2.11` | unselected additions | `get_available_thinking_levels`, `get_entries`, and `get_tree` are outside the selected 14 commands. |
| `18.2.11→18.3.0` | compatible internal | Adds `proc` provider resource routing; process mutation remains behind unselected `write`. |
| `18.3.0→18.3.1` | additive lifecycle and read resources | Adds `prompt_result`, `messageId`, `session_settled`, additive `get_state` data, and `cfg`/`attachment`/`conflict` read resources. Secrets are redacted and RPC writes remain unselected. |
| `18.3.1→18.3.2` | compatible tool internal | Session and grep internals change; tool schemas and commands stay. |
| `18.3.2→18.3.3` | compatible tool internal | Session-event and ask internals change; selected ask keys stay. |
| `18.3.3→18.3.4` | compatible session internal | Session state changes without selected command, frame, terminal, failure, or usage changes. |
| `18.3.4→18.3.5` | compatible session internal | Session state changes without selected command, frame, terminal, failure, or usage changes. |
| `18.3.5→18.4.0` | compatible tool internal | Session and ask internals change; selected keys and commands stay. |
| `18.4.0→18.4.1` | compatible tool and catalogue internal | Model registry and read/search internals change; exact selected provider/model binding and tool keys stay. |
| `18.4.1→18.4.2` | compatible tool internal | Search and read-summary internals change; selected names and keys stay. |
| `18.4.2→18.4.3` | compatible model and tool internal | Registry and read formatting change; selected provider/model binding and schemas stay. |
| `18.4.3→18.4.4` | additive state and unselected command | `get_state` gains optional queued-message state; `remove_queued_message` stays unselected. |
| `18.4.4→18.4.5` | compatible event-filter internal | Event filtering and cache warming remain outside the selected mapping. |
| `18.4.5→18.4.6` | compatible prompt admission timing | The correlated `prompt` response waits for message admission, not turn completion; `prompt` remains background-dispatched and control commands can overtake it. |
| `18.4.6→18.4.8` | compatible package hop | Selected command set and mapped-source subset stay; complete package delta is frozen. |
| `18.4.8→18.4.9` | compatible URI normalization and unselected additions | Internal URI normalization changes; `read` stays at its existing read tier and `ssh` stays exec-tier. New subagent and word-prediction commands remain unselected. |
| `18.4.9→18.4.10` | compatible input ordering | Prompt/steer/follow-up scheduling and abort invalidation change; response shape and ID correlation stay. |
| `18.4.10→18.4.11` | additive goal state and unselected commands | Goal/fork commands and goal state are outside the selected mapping; ordinary abort and terminal mapping stay. |
| `18.4.11→18.4.12` | unselected process and subagent internal | No selected command, read/write tool, or permission tier is added. |
| `18.4.12→18.5.0` | compatible session internal | Commands, schemas, response correlation, terminal, and failure mapping stay. |
| `18.5.0→18.5.1` | unselected live audio additions | Live audio commands and helpers remain unselected. |
| `18.5.1→18.6.0` | compatible RPC client internal | Client internals change without command, response, frame, or event-shape changes. |
| `18.6.0→18.6.1` | compatible session internal | Session internals change without command, terminal, failure, usage, or callback changes. |
| `18.6.1→18.6.3` | additive state and model refresh | `get_state` adds optional data; model lookup remains exact. Queue and btw operations remain unselected; frame limits stay. |
| `18.6.3→18.7.0` | compatible read and unselected auth additions | JSON read handling remains under existing `read`; login/logout stay unselected and model aliases remain outside this route claim. |
| `18.7.0→18.8.0` | wire-equivalent serialization optimization | Frame JSON shape and physical/logical limits stay; package inventory also covers read/search helper changes. |
| `18.8.0→18.8.1` | compatible model and session internal | Selected commands, tool keys, frame shape, terminal, and usage stay. |
| `18.8.1→18.8.2` | compatible model internal | Resolver metadata changes without selected commands or schemas changing. |
| `18.8.2→18.8.3` | compatible model picker fix | Model-picker/model-hub/agents-view repeat opening is fixed; selected commands and schemas stay. |

GitHub-only stable tags `18.0.2`, `18.1.7`, `18.4.7`, and `18.6.2` have no
published npm artifact and remain excluded. The older `17.4.3` and `17.4.4`
exclusions remain unchanged. `18.8.4` is the synthetic later-stable point;
at the identity observation it was not observed as a published artifact.

## Latest movement after the identity commit

The frozen identity was observed at `2026-10-08T02:28:44.734Z`. A fresh
official-channel re-probe at `2026-10-08T03:44:31.209Z` found npm `latest`
`18.8.4`, published at `2026-10-08T03:30:01.828Z`, with integrity
`sha512-MIiecZZbQT45Lnn1fJwq2gCG2+LBJwQ0cyQodqIPxnzcIRgawapLEaYZn/zXBo+0tn2oCZO6QiJrgKHe1OrsHg==`,
SHA-1 `eda73035c913b964d9c335e3b6bbf2a24520e749`, and `gitHead: null`.
GitHub's latest non-prerelease is [`v18.8.4`](https://github.com/can1357/oh-my-pi/releases/tag/v18.8.4),
published at `2026-10-08T03:25:18Z`, tag commit
[`40e9368ef0458fd9073329cdff4174895f91bc6b`](https://github.com/can1357/oh-my-pi/commit/40e9368ef0458fd9073329cdff4174895f91bc6b).
The channels agree on `18.8.4`.

This stable appeared after the identity commit, so Contract 029 records it as
`UnverifiedNewer` and keeps the qualified ceiling at `18.8.3`. Release notes
were checked for discovery and list changes across auth, usage, model
selection, and Tern UI, including a breaking `pi-ai` function signature; they
do not establish the selected RPC behavior. No `18.8.4` npm tarball was
downloaded or source tree classified, so no compatibility claim is made for
it. A follow-up family qualification must verify that exact npm artifact and
classify its selected-source hop before extending the claim.

## Selected RPC boundary and claim

The selected 14 commands remain `negotiate_protocol`, `set_model`,
`set_thinking_level`, `set_auto_retry`, `set_auto_compaction`,
`set_steering_mode`, `set_follow_up_mode`, `set_interrupt_mode`, `get_state`,
`get_available_models`, `prompt`, `steer`, `follow_up`, and `abort`. The
selected tools remain `read`, `grep`, `glob`, `todo`, and `ask` with unchanged
input-key sets. Physical frames stay capped at 1 MiB and reassembled logical
frames at 64 MiB. Strict LF JSONL, response ID/command/success/data, terminal
`agent_end.isTerminal == true`, provider failure
`assistant message_end.stopReason == error`, and input/output/cacheRead/cacheWrite
usage mapping remain unchanged.

Provider-internal `proc://`, `cfg://`, `attachment://`, and `conflict://`
resources are reachable through Oh My Pi's already selected ambient `read`
tool. The qualified adapter surface remains its existing tool set and
contract; Swallowtail exposes no URI-specific operation or host read/write
service. `ssh` remains exec-tier; `cfg://` redacts credential values;
configuration writes require settings approval; `proc://` writes use the
unselected exec-tier `write` tool. These provider-internal resource semantics
are bounded and recorded separately from the route's operation claims.

Extend the existing claim, `oh-my-pi.rpc.package-window-2`, as a compatible
extension: maintained `18.0.0..=18.8.3` on the same private
`oh-my-pi.rpc-v2-v18.0.0` behavior revision. Keep deprecated
`17.2.9..=17.4.2` on `oh-my-pi.rpc-v2-v17.2.9`, all four `18.x` unpublished
gaps, incompatible `17.4.3`/`17.4.4`, `AllowUnverified`, and the frozen
`oh-my-pi-rpc-17.2.9` decoder corpus. Official stable `18.8.4` remains
visible as `UnverifiedNewer` after the post-identity movement. The `pi.package`
and `pi.sdk-sidecar.package` axes remain separate.

No artifact was executed. There was no provider prompt, login, live catalogue
or session, credential use, installation, host update, workflow change, or
release action.
