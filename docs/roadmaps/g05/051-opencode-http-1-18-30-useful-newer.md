# g05.051 OpenCode HTTP 1.18.30 Useful Newer

Status: completed; OpenCode HTTP qualified through official `1.18.30` as a compatible `surface-19` extension; PR #314 merged at `2b79b083` after independent exact-head review `5633203211`
Owner: Tom
Created: 2026-09-11
Depends on: Contract 029; qualified OpenCode HTTP `1.18.29`; Research 292; g05.037
Vision tags: route currentness, OpenCode HTTP, compatibility

## Outcome

Qualify current official npm/GitHub `opencode-ai` `1.18.30` for the exact
`opencode.http` / `opencode.server` family if deterministic artifact evidence
admits it. Preserve identity-before-claim, the `1.14.48` baseline, every
historical segment and gap, the existing HTTP/SSE facade, and
`AllowUnverified`.

Official npm and GitHub stable were both `1.18.30` at planning. The only
published stable after the qualified `1.18.29` ceiling is `1.18.30`. The host
reports `1.18.18`; that is observation input, not qualification or install
authority.

## Ready-State Rubric

- [x] The operator flagged `1.18.29` -> `1.18.30` and explicitly said `Go`.
- [x] npm and GitHub agree on official stable `1.18.30`.
- [x] Current `opencode.server` is `AllowUnverified` through qualified
      `1.18.29` on `opencode.http-sse.surface-19`.
- [x] The single published hop, host observation, boundaries, evidence,
      validation, and stops are explicit.
- [x] No provider call, host change, release, or consumer mutation is required.

## Work

1. Re-probe npm and GitHub stable, observe the existing host safely, and freeze
   Research 304 plus a secret-free `1.18.30` identity corpus. Compare official
   `1.18.29` -> `1.18.30` npm artifacts and correlated GitHub tag trees. Do not
   assume linear Git ancestry: the planning-time tag comparison is diverged.
   Build deterministic complete inventories and classify every shipped file
   feeding mapped HTTP/SSE wire shape, provider catalogue, session lifecycle,
   callbacks, failure, usage, capability, and configuration behavior.
2. Inspect at minimum the changed `packages/opencode/src/provider/provider.ts`
   and `provider/transform.ts`, the added Astra system prompt and every selected
   route/handler/OpenAPI input. Commit identity evidence before any production
   claim edit. Name exactly one Contract 029 outcome: compatible extension,
   adapter-private milestone, new driver/facade stop, or evidence stop. Treat
   changelog text as subordinate evidence.
3. Only for an admitted segment, update `selection.rs`, mutation-sensitive
   tests, the OpenCode prepared guide, exact OpenCode route/feature-matrix
   version truth, `[Unreleased]`, standing-lane state, and one claim log in a
   second commit. Preserve all old gaps, claim IDs, and unrelated OpenCode
   surfaces unless the evidence requires a private milestone.
4. Re-probe official latest immediately before the identity commit and final
   push. Apply Contract 029's In-Run Latest Movement rule without weakening the
   identity-before-claim boundary. A stable first observed after the identity
   commit remains `UnverifiedNewer` and does not reopen this claim.

## Dispatch Manifest

| Field | Task 051 |
| --- | --- |
| Readiness | ready |
| Prerequisites | Contract 029; Research 292 and frozen OpenCode `1.18.29` evidence; clean pushed `main`; npm/GitHub consensus on `1.18.30` |
| Completion conditions | Research 304 and a secret-free corpus freeze exact `1.18.29` -> `1.18.30` identity; identity is committed before any claim edit; one segment outcome is recorded; an admitted outcome raises only the proved `opencode.server` claim and current downstream truth; a stopped outcome leaves claims unchanged; exact-head review and named validation pass |
| Owned mutable paths | `crates/swallowtail-adapter-opencode/src/selection.rs`; OpenCode selection and identity tests/fixtures under `crates/swallowtail-adapter-opencode/tests/**`; `docs/research/304-*.md` and one research-index line; `docs/guides/opencode-http-prepared-integration.md`; exact OpenCode rows/cells in `docs/guides/provider-route-matrix.md` and `provider-solution-feature-matrix.csv`; `CHANGELOG.md` `[Unreleased]`; one new OpenCode identity log and, if admitted, one OpenCode claim log plus their index lines; this task's result/status; the OpenCode currentness paragraphs in `docs/roadmaps/standing-lanes.md`; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, `docs/roadmaps/generation-index.md`, and the rest of `docs/roadmaps/standing-lanes.md`; queue coordinator edits these at closeout |
| Forbidden paths | OpenCode Contract 061 projection surfaces and Candidate L rows; OpenCode ACP and web-search work; every other adapter; contracts; historical research and release notes; any public API change |
| Approved concurrent siblings | none on shared OpenCode or closeout surfaces; unrelated implementation may proceed only when queue serialization protects reserved files |
| Worker capability class | evidence-first Rust currentness worker; npm/GitHub artifact retrieval and deterministic full-tree comparison; no provider credentials |
| Acceptance evidence | exact npm metadata/tarballs, GitHub tag/release identity, host version/digest observation, full artifact inventories and mapped-surface classification, mutation-sensitive fixtures/tests, and exact current-doc agreement |
| Review oracle | the smallest counterexample is linear-history inference despite diverged tags, an uninspected changed mapped file, a changelog-only compatibility claim, a lost historical gap, a range beyond the admitted segment, or current docs disagreeing with `selection.rs` |
| Stop conditions | official-channel disagreement; major-line reset; mapped public lifecycle/capability/authority change; unclassified catalogue/wire/failure delta; new driver/facade required; provider/live evidence required |
| Escalation owner | operator via Chatterbox for policy/authority; queue coordinator for mechanical blockers |

## Boundaries

One family only. No prompt, login, live HTTP/SSE server or session, provider
contact, credential use, package install, host update, OpenCode ACP or web-search
work, Contract 061 projection expansion, Gemini deferral lift, release, tag,
publication, or consumer mutation. Do not execute downloaded official
artifacts.

## Validation

- `cargo fmt -p swallowtail-adapter-opencode -- --check`
- `effigy validate:focused swallowtail-adapter-opencode`
- `effigy package:verify-affected swallowtail-adapter-opencode`
- `effigy check:examples`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:northstar`
- research, logs, roadmaps, g05, roadmap-number, status, and next-action checks
- `git diff --check`

## Acceptance

- [x] official identity is reproducible for the published hop
- [x] every changed shipped file feeding selected behavior is classified
- [x] identity evidence lands before any production claim edit
- [x] only the admitted segment changes the claim; a stop changes no claim
- [x] historical segments, gaps, claim ID, `AllowUnverified`, and unrelated
      OpenCode surfaces survive
- [x] official latest is rechecked at both required boundaries

## Result

Worker evidence: the admitted segment is a compatible extension of
`opencode.http-sse.surface-19` through official `1.18.30`. Research 304 and
`crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.30/` freeze
npm and GitHub identity for the single published hop `1.18.29` -> `1.18.30`.
The published GitHub tags diverge, so identity is proved from deterministic
complete source trees: the repository tree grows from 6557 to 6561 files with
four added paths, no removals, and 101 changed files, and
`packages/opencode/src` grows from 407 to 408 files. Selected HTTP/SSE route
and handler files and OpenAPI SHA-256 are byte-identical. Unmapped
provider-internal Bedrock model-id resolution, unmapped GitLab reasoning-option
shaping, the unmapped GPT-6 Astra system prompt and prompt text, provider SDK
dependency bumps, and the explicit-service-tier patch are the only shipped
deltas. Identity landed as its own commit before any production claim edit.

Claim: `OPENCODE_LATEST_QUALIFIED_VERSION` is `1.18.30`, baseline `1.14.48`,
claim id `opencode.http.server-window-1`, `surface-19`, every historical gap,
and `AllowUnverified` stay. Synthetic later stable `1.18.31` is the visible
`UnverifiedNewer` point. The prepared guide, both OpenCode route-matrix rows,
the feature-matrix version cell, the architecture ceiling lines,
`CHANGELOG.md` `[Unreleased]`, the standing-lane OpenCode paragraphs, and the
identity and claim logs agree with `selection.rs`. Official latest was
re-probed immediately before the identity commit and again immediately before
push; both remained `1.18.30`. No provider call, live server, install, host
update, release, tag, or consumer mutation occurred.

Independent exact-head review and the merge gate are queue-owned.

Merge: PR #314 (identity `6d4fbf44`, claim `f53cbe81`) passed independent
exact-head review `5633203211` with no findings and merged to canonical
`main` at `2b79b083`. Documentation closeout records the merged outcome.
