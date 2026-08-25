# g04.064 Kimi Code 0.38.0 Headless V2 Useful Newer

Status: ready
Owner: Tom
Created: 2026-08-25
Depends on: g04.032; g04.063; Contract 029 currentness
Research: 179, 210, 211 reserved

## Purpose

Qualify exact official npm `@moonshot-ai/kimi-code@0.38.0` on the
`kimi-code.headless` route's actual default agent-core-v2 `runV2Print` path,
or document an incompatible stop with the production claim unchanged.

This is one-family Contract 029 currentness work. It repairs the gap exposed by
Research 210. It does not reopen headless reasoning effort and does not close
g04.

## Current Boundary

`kimi-code.headless` currently qualifies exact `0.29.0..=0.37.2` under
`kimi.headless.stream-json.v1`. Exact `0.38.0` is permitted
`UnverifiedNewer`, but naked `kimi -p` dispatches to agent-core-v2
`runV2Print` unless `KIMI_CODE_LEGACY_FLAG` is truthy. Swallowtail does not set
that flag.

The lane must qualify the actual default v2 path. It must not force legacy v1,
inherit the old decoder claim without evidence, or flatten ACP or local-server
qualification onto headless.

## Acceptance

Identity:

- freeze exact official `0.38.0` v2 source and secret-free stream-json corpus
- map renderer, event grammar, terminal states, stderr, retry, tool activity,
  interruption, and retained-state semantics against the current decoder
- name one shape: adapter-private milestone, compatible extension, new public
  driver/facade revision, or incompatible stop
- make no production claim edit before the evidence decision

Claim:

- if Research 211 admits an adapter-private mapping, qualify exact `0.38.0`
  under a new `kimi.headless.stream-json.v2` behavior milestone
- update selection, preparation, decoder fixtures/tests, guides, matrices,
  changelog, research, logs, and indexes only to the proved boundary
- keep `0.29.0..=0.37.2` qualified under v1 and `0.38.1` synthetic
  `UnverifiedNewer`
- if evidence requires a public lifecycle change, live prompt, login, or
  provider-dependent inference, stop honestly and name the incompatibility

## Out Of Scope

- Kimi headless reasoning effort, plan, permission, raw environment, or config
- `KIMI_CODE_LEGACY_FLAG` as a compatibility workaround
- Kimi ACP, local-server, Platform Chat, or Python `kimi-cli` claim changes
- live account work, login, credentials, provider prompt, paid inference,
  package install, or host mutation
- official versions after exact `0.38.0`, other route families, release, merge,
  generation rollover, or g04 closure

## Validation

```sh
cargo fmt -p swallowtail-adapter-kimi
effigy validate:focused swallowtail-adapter-kimi
effigy package:verify-affected swallowtail-adapter-kimi
effigy qa:routes
effigy qa:northstar
git diff --check
```

Do not run live probes, consumer checks, release checks, or broad workspace QA.

## Batch Cards

- [179-kimi-code-0-38-0-headless-v2-identity.md](batch-cards/179-kimi-code-0-38-0-headless-v2-identity.md) — ready
- [180-kimi-code-0-38-0-headless-v2-claim.md](batch-cards/180-kimi-code-0-38-0-headless-v2-claim.md) — conditional

## References

- [Research 179 Kimi Code 0.38.0 Identity](../../research/179-kimi-code-0-38-0-identity.md)
- [Research 210 Kimi Code Headless Reasoning-Effort Evidence](../../research/210-kimi-code-headless-reasoning-effort-evidence.md)
- [Research 211 Kimi Code 0.38.0 Headless V2 Identity](../../research/211-kimi-code-0-38-0-headless-v2-identity.md)
- [Contract 029 Interface Version Qualification And Compatibility](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Version Currentness Checkpoint](../../guides/version-currentness-checkpoint.md)
- [Standing Lanes](../standing-lanes.md)
