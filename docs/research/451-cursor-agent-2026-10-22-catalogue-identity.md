# Research 451: Cursor Agent 2026.10.22 Catalogue Identity and Qualification

Status: complete identity evidence for `cursor-agent.catalogue`; the
production claim in this change extends the catalogue ceiling.

Observed 2026-10-26 on `cursor-agent.release-date`. The official ACP registry
current entry reports `2026.10.22` and publishes Cursor archive
`2026.10.22-27dcf5f` for Darwin ARM64 and Linux x64. Official registry main
at `ccea3c3011d3db7fca9aa294f4a539ce0377bc19` contains that same current
entry. Later main commits update non-Cursor previews, including
`06ecb81db3e8c5c1b9b7483651bd754555ebe963` and
`84cab0610409c66ccebe15d37b196cc26525ba39`, and leave the Cursor entry at
`2026.10.22-27dcf5f`. The Cursor entry's last commit remains
`61a3cb3f49a35c40ce8ac4222192a7b7c8be5998`. A re-probe immediately before
this record landed still publishes that archive. Downloaded
artifact identities and complete inventories are frozen in the
[identity fixture](../../crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-2026.10.22/README.md).

## Official hop ledger

The selected channel is the official ACP registry Cursor entry and its
versioned archive URLs. Every published hop after the catalogue ceiling
`2026.10.01-14929f9` is included:

| Version | Registry commit | Commit date | Role |
| --- | --- | --- | --- |
| `2026.10.14-0f9edde` | `8d7d20369fcaa4ac943df97d1e5502b78681a003` | 2026-10-14 | superseded same-date build |
| `2026.10.14-941492e` | `4d1729af836732b46840080920912dac7d5a2935` | 2026-10-16 | qualified milestone |
| `2026.10.16-e3cb207` | `d043ba62f22f4036d7cdc95e5870b3ac70cc01ed` | 2026-10-17 | qualified milestone |
| `2026.10.18-ea085cb` | `6e20ddf94b078f0385b56c90aba40107edad2748` | 2026-10-19 | qualified milestone |
| `2026.10.19-6bbf2c4` | `a83f6b7f0c60c3b87f45dff3b12603b4cae19dd0` | 2026-10-20 | qualified milestone |
| `2026.10.22-27dcf5f` | `61a3cb3f49a35c40ce8ac4222192a7b7c8be5998` | 2026-10-23 | qualified milestone |

Registry dates were used for discovery. Every selected point is bound to its
exact archive digest and build revision. No later Cursor update is in the
registry history at observation. No calendar gap is inferred. `cursor-agent`
was not on PATH.

| Version | Darwin ARM64 archive SHA-256 | Linux x64 archive SHA-256 | Darwin files | Linux files |
| --- | --- | --- | ---: | ---: |
| `2026.10.14-0f9edde` | `efc39839ee3bc0b05467cd6e4baf23a43e358dae61d11512d9c5bf4ebbb3217b` | `74fac931fe3578980d070701872b7dab54eb87a9abf9b6e2e6be19beeb9167a7` | 450 | 458 |
| `2026.10.14-941492e` | `c6d9e2eaa1b1e24221517e0b4206b1f1593792e6f0d8a52e9b507bbe7ab0cc77` | `6f6bf39dd2df29dcd38643cdfab0155ce71699953889f732deec123f6562bb2d` | 450 | 458 |
| `2026.10.16-e3cb207` | `5f04d196a84ddb5c20a4a466bdd74e3695bfb447e3c4a1e6071364b2b9497dd0` | `3aa3f48cd7a3909a98c9b3f2a4085f0c021088bf826690b22c61f424c843e552` | 452 | 460 |
| `2026.10.18-ea085cb` | `94806e2d28ba57e4206319e2e56a93a60263744a176dafc533ce633ee4a7a0b0` | `22325b799e8b7a8a94fc79d3596cbfa1b85407343d34c3a552f703d4023b1c79` | 451 | 459 |
| `2026.10.19-6bbf2c4` | `095dccb923488d280aae697ba68c13a92bbd9ce1dc6105401e147e25869f2b13` | `cf80cc18d8b33bf33e762353977ca1868cfc45290c92c0ffdcef83bb8b37b44f` | 452 | 460 |
| `2026.10.22-27dcf5f` | `2f5b0a3dd5fb951cecffac813867d8a08d810e59837d59fd5f12d69b922398d7` | `85296944e4600a83c4b764f578d1502bf9c416db200d33a593e8f3ecabfdc294` | 437 | 445 |

The prior qualified ceiling archive is the first tree in the inventory, so
each adjacent hop is classified independently. The ceiling Darwin and Linux
archive digests match Research 393: `778d04e542adc5c8b6760fda3ebe0757f903b1764f2792c232ef9a35e6e2151b`
and `ba9a855f8f813c91b9f2707127572d2dc9ae5a62818e1c36719625d0fb8bd452`.

## Selected catalogue surface

The command definition remains
`.command("models").description("List available models for this account")`.
The selected handler remains `handleModelsList` and keeps the account options
`endpoint`, `apiKey`, `authToken`, and `insecure`. It still builds the
available model set with `useModelParameters=true` and `doNotUseMarkdown=true`,
then prints the same `Available models` heading and rows. The adapter parser
maps model identity and visible display text. It extracts default status only
from the exact `(current, default)` suffix and exposes no separate current
field. A nonzero process exit still fails as `catalogue_exit_failed`.

Linux `./src/commands/models.ts` is byte-identical from `2026.10.01-14929f9`
through `2026.10.19-6bbf2c4`, including both `2026.10.14` builds. At
`2026.10.22-27dcf5f` the failure string passed to stderr goes through an
unknown-guard helper before `String`. Renderer literals, marker order
(`current` then `default`), and `process.exit` codes stay. The registration
stops wrapping `insecure` and `authless` in `Boolean`; `handleModelsList`
still applies `Boolean` to `insecure`, and the adapter sends argv `models`
with no flags. The model-utils clone helper constructs a protobuf message
with the same printed fields. Model-service request literals stay identical
at every hop. Quoted string literals of the Linux models, service, utils,
and registration modules add only `./src/utils/unknown-guards.ts` and the
protobuf import at `2026.10.22`.

The shell launcher stays byte-identical. Embedded Node changes only at
`2026.10.22` and was not executed. From `2026.10.16` `package.json` records
private native dependency versions under `@anysphere/agent-cli-runtime`.
That package is not the npm `cursor-agent` axis. The local-provider branch,
team and worker commands, and interactive model picker stay unmapped. A root
`--repo <url>` required option disappears at `2026.10.22`; the `models`
command itself gains no option.

## Contract 029 decision

This is a compatible extension of the existing
`cursor-agent.catalogue.calendar-release-v1` behavior and
`cursor-agent.catalogue.release-window-3` claim. Keep baseline
`2026.07.01-41b2de7`, the twelve previously qualified points, and older
independently unqualified published gaps `2026.08.25-3e8eec8` and
`2026.09.08-6caf4ff`. Add `2026.10.14-941492e`, `2026.10.16-e3cb207`,
`2026.10.18-ea085cb`, `2026.10.19-6bbf2c4`, and `2026.10.22-27dcf5f`.
Reject superseded `2026.10.14-0f9edde` with the existing exact build gate.
That published build previously classified as unverified newer and now fails
binding; the rejection is the required exact-build narrowing. Selected
catalogue stdout, argv, and mapped fields do not narrow. No new public
operation, behavior revision, or authority is introduced. ACP and headless
remain at `2026.10.01-14929f9`.
The first unpublished later date is `2026-10-23`.

Archives were hashed and inspected statically, not executed. No authenticated
catalogue, provider session, prompt, credential, installation, or host update
was used.

Official sources: [ACP Registry current metadata](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json), [Cursor ACP registry entry](https://github.com/agentclientprotocol/registry/blob/main/cursor/agent.json), [registry commit history](https://github.com/agentclientprotocol/registry/commits/main/cursor/agent.json).
