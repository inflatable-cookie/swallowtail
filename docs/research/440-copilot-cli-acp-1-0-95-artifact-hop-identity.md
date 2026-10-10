# Research 440: Copilot CLI ACP Artifact Hops Through 1.0.95

## Status

Frozen provider-free artifact evidence; no route claim change.

## Question

What exact wrapper and Darwin ARM64 package artifacts were published after the
`copilot-cli.acp` ceiling `1.0.80`, and can their selected ACP behavior be
classified through the current npm stable?

## Official identity

The official `@github/copilot` npm `latest` and
`@github/copilot-darwin-arm64` `latest` tags both resolved to `1.0.95` at
`2026-10-10T13:59:23Z`. Both prerelease tags resolved to `1.0.96-2`. The exact
wrapper and Darwin ARM64 artifacts, signatures, complete file trees, and
published times for `1.0.80` through `1.0.95` are in the
[artifact hop ledger](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-currentness-1.0.95/artifact-hop-ledger.json).
The test fixture's [README](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-currentness-1.0.95/README.md)
describes the method and stop.

The current wrapper tarball SHA-256 is
`838be5537db8bd0f7c7c2e555063770ab559249cf2c3ff3c23b2efc81c854779`; its npm
SRI is
`sha512-TAYlgMwjTnHi04ZGRc6Z+41piGeUC6xuA8z7gWVc5qOTqJLi/B3vCVg9ttkwvxnahbTWjX8x0DORwrJMXOQxGg==`.
The current Darwin ARM64 tarball SHA-256 is
`95d49e3023921f0bf694e75e17591b68f8b1e59942c2606e1bc1379828d72b2f`; its npm
SRI is
`sha512-rh4KQhjg2uO/fJHlDkOQh3R2etNRsnQ22vLbcJcEObFVrc449m6g3injew08QhDQQT16fP+ADfPRcGxcy74P6g==`.
The complete tree digests are `25aaeacf9b3f5e44639d981a673ceefa307b672e78b62b53f987414fc6ef80cf`
and `cd9b2c14b680f752cd6d3e2643eadd024f8c66e38225d0fbc69e4485aee2edaf`,
respectively. The wrapper contains four files; the native package contains
four files, with `package/copilot` accounting for the opaque runtime.

## Published-hop classification

Every stable hop from `1.0.81` through `1.0.95` is present. The wrapper
`npm-loader.js` is byte-identical across the window. Its root package manifest
changes on each hop. Native per-hop file counts are:

| Hop | Added | Removed | Changed | Identical | Selected ACP classification |
| --- | ---: | ---: | ---: | ---: | --- |
| 1.0.80 → 1.0.81 | 5 | 21 | 27 | 193 | Runtime files changed; unresolved |
| 1.0.81 → 1.0.82 | 0 | 1 | 19 | 205 | Application and runtime files changed; unresolved |
| 1.0.82 → 1.0.83 | 7 | 3 | 48 | 173 | Application and runtime files changed; unresolved |
| 1.0.83 → 1.0.84 | 4 | 0 | 37 | 191 | Application and runtime files changed; unresolved |
| 1.0.84 → 1.0.85 | 0 | 228 | 3 | 1 | Repackaged to opaque executable; unresolved |
| 1.0.85 → 1.0.86 through 1.0.94 → 1.0.95 | 0 per hop | 0 per hop | 2 per hop | 2 per hop | Opaque executable changes each hop; unresolved |

The ledger contains exact file paths and hashes for each set, plus a
classification for every added, removed, or changed file. Runtime/application
files are not byte-identical across the selected window. The `1.0.85` package
layout removes the previous source/module tree and ships a changed monolithic
binary on each later hop. Changelogs and package metadata do not resolve that
selected-surface gap.

## Route and permission gates

`selection.rs` still pins `1.0.80 QualifiedOnly`; the production driver uses
the approved executable and `--acp --stdio`. Research 439 is accepted only for
the reject-all/cancel gate at exact `1.0.93`. That original ran the direct
native executable with `--model auto --acp --stdio` and child-only package
update/cache controls. Its model was unobserved, and the attributable execute
action did not match the requested sentinel edit. It does not establish the
production launch tuple, edit safety, `1.0.81`, or later releases.

No artifact was installed or executed. No provider turn, login, credential,
host change, mapping change, or claim change occurred. The remaining shared
prompt slot is preserved.

## Decision and next evidence

Keep the existing exact `1.0.80` claim and every unqualified point. The
artifact identities are now frozen, but all 15 published hops have an
unresolved native runtime change. Before widening the route, obtain
authoritative versioned ACP implementation evidence for the production launch
tuple, or request a separate ruling for one route-matched attempt at the exact
current stable. That evidence must cover the correlated pending execute and
`requestPermission` action, the exact selected action shape, cancellation
result, and no-effect outcome. Do not infer edit safety from cancellation.

## Validation

`effigy validate:current-copilot-cli-acp` validates this immutable identity and
hop ledger. The prior run's fake-only driver validation remains the evidence
for the existing prepared route; it is not transferred to newer vendor
artifacts.
