# g05.052 Grok Build 1.0.25 Model Catalogue

Status: ready
Owner: Tom
Created: 2026-09-12
Depends on: Contracts 020 and 047; released `v0.5.0`; Desktop g02.086 producer intake
Vision tags: model catalogue, Grok Build, prepared operations, consumer truth

## Outcome

Add a separately prepared Grok Build model-catalogue operation for the exact
installed `1.0.25` CLI when provider-free artifact evidence proves that
`grok models` is an admissible non-prompt source. Return ordered common
`ModelCatalogEntry` rows without widening the ACP execution claim or treating
session-negotiated model options as pre-session catalogue authority.

## Ready-State Rubric

- [x] The operator supplied and approved the Desktop producer intake on
      2026-09-12.
- [x] Installed Grok `1.0.25` (`f7e67d6988e2`, stable; SHA-256
      `9ef4a40ad60c6a5178a65caf39c2a148e6a98d0d2d350b10329dee34d9195d9c`)
      exposes a root `models` command described as “List available models and
      exit”.
- [x] Static binary evidence contains an ordered default-model document with
      `grok-4.6` first/default and `grok-4.5` second, plus source-owned display,
      description, context-window and reasoning fields.
- [x] Contracts 020 and 047 already separate catalogue observation from
      inference, session options, provider identity and consumer enablement.
- [x] The common model catalogue vocabulary preserves ordered exact IDs and
      optional source-supplied metadata; no core vocabulary expansion is
      currently required.
- [x] No provider call, prompt, credential read, host update, release, tag or
      Desktop mutation is authorized.

## Work

1. Freeze Research 305 and a secret-free exact `1.0.25` corpus from static
   installed/official artifact inspection. Prove the command and output grammar
   used by `grok models`; do not execute the catalogue command. If static
   evidence cannot tie the command to a bounded parseable output, record the
   typed non-admission ruling and stop without a catalogue claim.
2. For an admitted seam, add an exact `1.0.25` catalogue compatibility claim
   and separately prepared `DriverRole::ModelCatalog` operation. Bind the
   approved executable/environment, delegated Grok subscription access
   readiness, request ID, deadline, process/time services and exact catalogue
   behavior. Keep `grok_build_acp_claim`, `grok_build_model_for_version`, ACP
   model choice and every released execution segment unchanged.
3. Run only the exact non-prompt catalogue argv, close stdin without writing a
   prompt, bound combined output, parse strictly, require successful exit and
   join cleanup on success, failure, timeout and cancellation. Do not open ACP,
   create a provider session, send a turn, enable tools/extensions, persist a
   session, retry, update the CLI or fall back to an execution default.
4. Project provider order, exact opaque model ID, source-supplied display name,
   top-level default and only semantically aligned description/capability data
   into common `ModelCatalogEntry` rows. Preserve unknown future IDs. Reject
   malformed, duplicate, empty, over-limit and ambiguous-default catalogues.
   Do not derive a name from an ID or attach a capability the source omitted.
5. Add deterministic fake-process fixtures for the accepted document, unknown
   IDs, sparse optional metadata, auth/access rejection, malformed output,
   non-zero exit, deadline, cancellation and cleanup failure. Update the Grok
   prepared guide, exact route/feature truth, `[Unreleased]`, one claim log and
   this task result. Return the exact merged Swallowtail source SHA and every
   accepted contract/fixture path directly to Desktop Chatterbox.

## Dispatch Manifest

| Field | Task 052 |
| --- | --- |
| Readiness | ready |
| Prerequisites | clean pushed `main`; Contracts 020 and 047; exact installed Grok `1.0.25` identity; operator-approved Desktop producer intake |
| Completion conditions | Research 305 freezes static command/output evidence before any catalogue claim; either one exact `1.0.25` prepared catalogue operation lands with bounded provider-free conformance, or a typed non-admission ruling lands with claims unchanged; exact-head review and named validation pass; Desktop receives the closeout capsule |
| Owned mutable paths | `crates/swallowtail-adapter-grok/src/**`; Grok tests and new exact `1.0.25` fixtures under `crates/swallowtail-adapter-grok/tests/**`; `docs/research/305-*` and its index line; `docs/guides/grok-build-prepared-integration.md`; exact Grok rows/cells in `docs/guides/provider-route-matrix.md` and `provider-solution-feature-matrix.csv`; `CHANGELOG.md` `[Unreleased]`; one Grok claim log and its index line; this task result/status; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, `docs/roadmaps/generation-index.md`; queue coordinator edits these at closeout |
| Forbidden paths | Desktop; `grok_build_acp_claim` execution segments except tests proving immutability; other adapters; release notes; tags; publications; host credentials or config; any live-provider evidence |
| Approved concurrent siblings | Desktop g02.085 and g02.086 may proceed on their separate common projection/status scope; no sibling may edit this Grok adapter or reserved closeout surfaces without queue serialization |
| Worker capability class | evidence-first Rust adapter worker; static binary/artifact inspection; prepared-driver and bounded-process conformance; no provider credentials |
| Acceptance evidence | exact `1.0.25` executable/build/digest and official artifact identity; static command-to-output proof; frozen fixtures; command-shape, access, parsing, bounds, deadline, cancellation and joined-cleanup tests; exact current-doc agreement |
| Review oracle | the smallest counterexample is running a live catalogue/provider call, parsing negotiated ACP options, deriving the list from `grok_build_model_for_version`, inventing absent metadata, losing source order/default, admitting an unproved version range, or changing ACP execution compatibility |
| Stop conditions | static evidence cannot prove an independent parseable `models` seam; the command can send a prompt or inference turn; auth cannot remain separate from credential disclosure; output is unbounded/interactive; common projection would require invented semantics; a live provider call is required |
| Escalation owner | operator via Chatterbox for provider-call, release/tag, consumer-pin or policy authority; queue coordinator for mechanical blockers |

## Boundaries

This is a catalogue feature qualification for exact Grok `1.0.25`, not a
Contract 029 ACP execution upgrade. No `grok models` execution, provider
request, prompt, login, credential read, model turn, host install/update,
Desktop edit, dependency pin, release, tag or publication. The historical ACP
capsules' negotiated model options are explicitly inadmissible as pre-session
catalogue evidence.

An accepted merged source SHA may be offered to Desktop for a separately
authorized exact-source pin. Released `v0.5.0` remains immutable and does not
contain this seam. A new source tag remains a separate operator decision.

## Validation

- `cargo fmt -p swallowtail-adapter-grok -- --check`
- `effigy validate:focused swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-grok`
- `effigy check:examples`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:northstar`
- research, logs, roadmaps, g05, roadmap-number, status and next-action checks
- `git diff --check`

## Acceptance

- [ ] exact `1.0.25` static evidence precedes and bounds any catalogue claim
- [ ] preparation requires exact version, access readiness and immutable
      `ModelCatalog` plan evidence
- [ ] execution is one bounded `models` process with zero prompt/session/retry
- [ ] rows preserve provider order, exact IDs, default and only supplied
      metadata; unknown IDs pass through
- [ ] malformed/access/exit/deadline/cancellation/cleanup outcomes stay typed
- [ ] ACP execution choice and compatibility history remain unchanged
- [ ] final capsule names exact source SHA, Contract 020/047 paths, exact
      fixture paths, focused selector results and Desktop's exact-source versus
      later-release adoption boundary

## Next Task

After merge, Desktop Chatterbox may plan a separately authorized exact-source
adoption into g02.086. No release or tag follows automatically.
