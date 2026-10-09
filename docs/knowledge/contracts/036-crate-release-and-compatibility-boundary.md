# 036 Source Release And Compatibility Boundary

Status: active
Owner: Tom
Updated: 2026-10-09

## Purpose

Define Swallowtail's public package set, pre-1.0 compatibility promise, MSRV,
source-release evidence, consumer proof, and release authority.

Contract 029 separately governs provider, harness, SDK, protocol, service, and
runtime-interface versions.

## Initial Distribution

The initial external release is one annotated Git tag, `v0.1.0`, on the
canonical GitHub repository.

It is not:

- a crates.io publication
- a GitHub Release object
- a binary or sidecar bundle
- an installer
- an API 1.0 promise

All workspace packages declare `publish = false` for this release line.
Changing that posture requires a separate registry contract, package-name and
owner checks, archive evidence, and explicit operator authorization.

Consumers select exact packages from the same exact Git tag. They do not use a
moving branch, an untagged revision presented as a release, or an unpublished
registry fallback. Normal internal path dependencies remain inside the tagged
workspace checkout.

## Public Package Set

The immutable `v0.1.0` and `v0.1.1` tags contain 27 public source packages.
`v0.2.0` contains 28, adding the reviewed `swallowtail-adapter-muse` package.
The `v0.3.0` / `v0.3.1` tags keep those 28 packages and 34 production routes.
`v0.3.2` adds two reviewed additive packages,
`swallowtail-adapter-command-code` and `swallowtail-idioms`, for 30 packages
and 36 production routes.
`v0.3.3` additionally carries the reviewed additive
`swallowtail-adapter-deepseek-harness` package and
`deepseek-harness.jsonrpc` and `deepseek-harness.local-server` routes, plus
the reviewed additive `swallowtail-adapter-zcode`,
`swallowtail-adapter-cline`, `swallowtail-adapter-goose`,
`swallowtail-adapter-copilot-cli`, `swallowtail-adapter-mistral-vibe`, `swallowtail-adapter-qoder`,
`swallowtail-adapter-openhands`, `swallowtail-adapter-kiro`, and
`swallowtail-adapter-deepagents`. The tag is 40 packages and 47
production routes. OpenHands Agent Server is deferred: that package exists
without a production route. The additive packages are not part of the
immutable `v0.3.2` inventories.

Foundations:

- `swallowtail-core`
- `swallowtail-runtime`

Support:

- `swallowtail-host-local`
- `swallowtail-idioms`
- `swallowtail-testkit`

Protocols and transport:

- `swallowtail-protocol-acp`
- `swallowtail-protocol-openai-chat`
- `swallowtail-transport-acp-remote`

Opt-in adapters:

- `swallowtail-adapter-alibaba-model-studio`
- `swallowtail-adapter-anthropic`
- `swallowtail-adapter-antigravity`
- `swallowtail-adapter-bedrock`
- `swallowtail-adapter-claude-agent`
- `swallowtail-adapter-cline`
- `swallowtail-adapter-codex`
- `swallowtail-adapter-command-code`
- `swallowtail-adapter-copilot-cli`
- `swallowtail-adapter-cursor`
- `swallowtail-adapter-deepagents`
- `swallowtail-adapter-deepseek`
- `swallowtail-adapter-deepseek-harness`
- `swallowtail-adapter-gemini`
- `swallowtail-adapter-goose`
- `swallowtail-adapter-grok`
- `swallowtail-adapter-kimi`
- `swallowtail-adapter-kimi-platform`
- `swallowtail-adapter-kiro`
- `swallowtail-adapter-llama-cpp`
- `swallowtail-adapter-mistral-vibe`
- `swallowtail-adapter-muse`
- `swallowtail-adapter-opencode`
- `swallowtail-adapter-ollama`
- `swallowtail-adapter-oh-my-pi`
- `swallowtail-adapter-openai`
- `swallowtail-adapter-openhands`
- `swallowtail-adapter-pi`
- `swallowtail-adapter-qoder`
- `swallowtail-adapter-qwen`
- `swallowtail-adapter-xai`
- `swallowtail-adapter-zcode`

There is no umbrella crate or intentionally private implementation crate.
Every package remains separately selectable. Selecting one grants no authority
to use another, install a harness, acquire a model, start a server,
authenticate, select billing, or claim provider support.

A package addition, removal, merge, or private role requires architecture and
contract review before manifest work. Additive candidates receive explicit
source inventory and semantic API evidence before their first tag. Historical
tag package, dependency, API, route, and release-note inventories remain
immutable.

## Windows Compatibility Qualification

Tom ruled on 2026-10-09: “We ultimately haven't claimed _any_ windows
compatibility at all yet.. rather than letting Antigravity dictate it, I would
rather choose a time to do a full sweep across the whole codebase and do it
properly!” (decision `bf37c540-9f41-4e8c-bbd7-b19f0b6db096`).

There is no general Windows compatibility guarantee for the source release.
Windows qualification is a separate whole-codebase effort, scheduled by Tom.
An individual provider's Windows artifacts, upstream support or static evidence
do not establish Swallowtail support. The sweep must assess core/runtime, host
process ownership and cleanup, transports, adapters, paths and environment,
packaging and consumer preparation, with architecture-specific evidence and
explicitly documented route limits before any Windows claim.

Tom subsequently ruled on 2026-10-09: “Can we complete the antigravity work
without the windows component of it? Remove windows from the requirements so
we can return to it later? I'll look into copilot” (decision
`560bf22e-10b5-4fbf-96ab-c3154b1ec20f`). This lifts the whole Antigravity task
deferral: its non-Windows work may continue, while Windows proof remains in the
later whole-codebase sweep and is not a requirement for that task's completion.

Preserve existing exact qualified points, exclusions, retry settings and
retained evidence. This scope change adds no Windows claim and waives no
non-Windows proof: resource, retry, authentication and platform-specific
control-flow gaps must be settled before corresponding qualification. An
independently reviewed finite evidence result may identify the next exact proof
without increasing an unproved claim. Keep that follow-up explicit; task
completion is not qualification. Future execution still requires the approved
environment and authority. No VM action or provider execution follows from this
scheduling ruling.

## Dependency Topology

Normal internal dependencies remain acyclic across three layers:

1. core and protocol codecs
2. runtime
3. host support, testkit, remote ACP transport, and adapters

Each normal or build internal dependency declares both the workspace path and
an ordinary compatible requirement for the coordinated version. Path-only
development dependencies are permitted.

The version requirement does not imply registry availability. It preserves
the package relationship and leaves a future registry decision explicit.

## Coordinated Pre-1.0 Version

All packages use one coordinated workspace version. The first release is
`0.1.0`.

Before 1.0:

- compatible public API and guaranteed-behavior changes advance the patch
- breaking public API or guaranteed-behavior changes advance the minor
- provider-interface qualification remains a separate Contract 029 axis

Patch-compatible changes may include additive public items, internal
refactoring, safety fixes preserving documented behavior, additive safe
diagnostics, and newly qualified provider-interface versions.

The approved bounded Opaque claim implementation preserves released method
signatures and adds `has_version_interval`, but it also refuses some
previously accepted constructor inputs: more than 32 raw exclusion yields,
Opaque version or exclusion text over 256 UTF-8 bytes, and behavior-revision
text over 256 UTF-8 bytes. Duplicate exclusion yields count toward the bound.
This behavior narrowing means the overall change is not universally
patch-compatible. Classify the implementation as a pre-1.0 minor change;
Contract 029 route claims and qualifications remain separate.

Immutable `v0.3.3` was the prior tagged release. Source after that tag removed
the previously guaranteed but unqualified `minimal` reasoning value from exact
GPT-5.6 `openai.background` preparation. That guaranteed-behavior shrink is
breaking under this contract: the next source release from current `main` must
advance the coordinated pre-1.0 minor rather than use `0.3.4`. This
classification does not select, tag, or authorize a release; the eventual
release notes must name the removed value, the corrected opaque facade point,
and the fail-before-effects upgrade behavior.

Breaking changes include removing or incompatibly changing public items,
raising MSRV, shrinking a guaranteed provider range, removing a capability or
verified target, changing route identity, or weakening lifecycle, cleanup,
access, isolation, or evidence truth.

An urgent security exception must be operator-approved and record affected
packages, compatibility loss, rollback, and upgrade path.

## Rust And Target Support

The immutable `v0.1.x` verified floors are:

- Rust `1.90.0` for every package except Bedrock
- Rust `1.94.1` for `swallowtail-adapter-bedrock`

Bedrock's historical higher floor follows its pinned AWS SDK graph. `v0.2.0`
deliberately replaces that split with one Rust `1.95.0` floor for all
packages. This is a breaking MSRV raise and therefore uses a new pre-1.0 minor
version. The `v0.3.0` candidate retains the same floor.

The `v0.3.0` candidate must pass:

- all current-source packages at Rust `1.95.0`
- the complete workspace on the selected current stable toolchain

Apple Silicon macOS is the initial verified target. Other targets may work and
remain unverified, not prohibited.

Raising a floor or removing a verified target is breaking.

## Package Metadata

Every package declares or inherits:

- version
- edition
- license
- repository
- description
- readme
- `publish = false`
- Rust version

Metadata must describe the package's real role. It must not contain local
absolute paths, credentials, private endpoints, mutable provider payloads as
authority, or consumer product policy.

## Source Contents

The tag targets one clean non-root commit already present on the canonical
branch and approved remote. The tagged tree is the release artifact.

It may contain source, bounded deterministic fixtures, public documentation,
examples, license material, the dependency lock, and release validation
scripts.

It must not contain secrets, authentication state, mutable caches, build
output, generated local release bundles, developer-local paths, or unreviewed
live provider captures.

A deterministic source bundle may be retained as evidence. It must reproduce
the exact tagged commit and cannot replace canonical Git history.

`.crate` archives, registry publication order, registry size limits, and
registry owner state are outside the initial source-tag acceptance boundary.
Historical candidate evidence remains historical and must not be presented as
the current release candidate.

## Public API And Documentation

The first tag creates the compatibility baseline for its 27 packages. An
additive post-tag package receives separate candidate API evidence until an
operator authorizes a later source release containing it. `v0.2.0` retains
the 27-package `v0.1.0` baseline and adds Muse's first baseline without
rewriting the earlier inventory. The `v0.3.0` baseline sanctions the breaking
`Option<InterfaceVersionBinding>` return from `codex_cli_binding` and
`ollama_runtime_binding`; the package and route inventories remain unchanged.

Before the tag:

- publicly reachable items are reviewed as supported API or made private
- supported API has meaningful Rustdoc
- workspace documentation builds with missing-public-documentation denied
- normal-path examples compile
- an API baseline is generated from semantic Rust API evidence, not source-line
  hashes alone
- the changelog and release notes describe the actual tagged source

Mechanical comments that repeat an identifier do not satisfy documentation.
Examples and route guides supplement Rustdoc; they do not replace it.

Subsequent compatible candidates compare against the tagged semantic baseline
and receive explicit compatible or breaking classification.

A version-labelled package, route, dependency, or semantic API baseline records
the immutable tagged source for that version. Post-tag additions belong in the
next candidate's baseline, even while the workspace manifest still names the
older version. If current-source validation temporarily updates an older
version-labelled baseline after its tag, the next candidate must restore that
file byte-for-byte from the tag before generating the new baseline. This is a
correction back to tagged evidence, not authority to revise release history.
The exact tag comparison must prove every restored byte; unrelated historical
release notes and baselines remain immutable.

The semantic inventory uses `cargo-public-api 0.52.0` with
`nightly-2026-08-05`, all package features enabled, and blanket, auto-trait,
and auto-derived implementations omitted. That nightly exists only to produce
rustdoc JSON. It does not change the stable release compiler or either verified
Rust floor.

## Dependency And Security Evidence

A candidate requires:

- a committed dependency lock
- no known unaccepted vulnerability in the selected normal dependency graph
- an explicit license and source policy
- review of duplicate major protocol or TLS stacks where they change risk or
  maintenance cost
- currentness review for direct dependencies without blind upgrades

Security findings cannot be reclassified by omission. An accepted exception
must name reachability, affected packages, expiry or recheck condition, and
operator approval.

## Consumer Evidence

The release candidate must prove normal public paths without editing consumer
repositories by default.

Required evidence:

- an isolated external Cargo consumer using exact source identity
- deterministic prepared-facade execution for applicable route families
- complete route and feature guide coverage
- at least one operator-selected working application smoke through a normal
  authenticated product path
- exact upgrade and rollback instructions

The external Cargo smoke uses the candidate commit before tagging. After tag
creation, tag identity must resolve to that same commit.

Live credentials, provider calls, workspace writes, and consumer mutations
remain separately gated. Deterministic adapter or consumer scenarios own
repeatable lifecycle claims. A native application smoke proves integration,
not every provider behavior.

## Deterministic Candidate Gate

Credential-free release checks sit behind explicit Effigy selectors and cover:

- clean source and exact commit identity
- 40-package `v0.3.3` metadata and dependency topology, kept distinct
  from the immutable 30-package `v0.3.2`, 28-package `v0.2.0` / `v0.3.1`, and
  27-package `v0.1.x` baselines
- semantic public API baseline, with the 40-package `v0.3.3` release frozen
  separately from historical `v0.3.2` files and the immutable
  28-package `v0.3.0` compatibility baseline
- denied missing public documentation
- dependency advisory, license, and source policy
- Rust `1.95.0` floor and current stable
- formatting, lint, tests, guide coverage, and examples
- external source-consumer compilation and normal-path preparation
- release-note, changelog, license, and security-policy presence

The repository-owned Effigy release configuration is authoritative for
candidate gates, version preparation, and tag execution. It targets the
virtual workspace version explicitly and carries no registry or GitHub Release
step. Swallowtail-specific scripts supply package, API, floor, security, and
external-source evidence behind that configuration.

The configuration retains Effigy's first-tag/current-version setting as
historical bootstrap authority for `v0.1.0`. Later candidates still require a
strictly greater version and an absent matching tag. The setting does not
permit a lower version, a repeated release, or a bypass around normal release
gates.

The gate performs no authenticated provider work and no external release
mutation.

Provider-free candidate preparation is not a one-shot resource. A failed
prepare in a clean isolated worker checkout may be diagnosed, repaired within
the task's existing paths and acceptance boundary, and rerun. Preserve the
failed receipt or exact diagnostic and prove the final candidate diff; do not
require a disposable rehearsal checkout merely to avoid consuming local
authority. One-shot limits remain appropriate for provider calls and external
release mutations such as tag creation or push.

## SDK Patch And Currentness Release Separation

Tom's 2026-10-08 board answer to decision
`6fd8ba99-46d5-4442-9f40-d79199034bc4` is “Separate urgent SDK patch; sweep in
minor”. Prepare a separately reviewed `v0.5.2` SDK correction candidate from
the released `v0.5.1` source at
`e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27`. Include only patch-compatible
registered-tool lifetime and usage corrections, their proved necessary
dependencies, and corresponding release evidence. Preserve the released
consumer API, route versions and behavior guarantees unless an exact
patch-compatible extension is independently established. Backport dependency
closure and compatibility must be proved; reviewed changes on `main` alone
do not establish an isolated patch candidate.

The full approved currentness sweep continues toward a pre-1.0 minor release.
Its evidence stops, adaptation obligations and operator gates remain; none
is waived or abandoned. This split replaces the requirement to complete that
full sweep before the urgent SDK patch. Breaking behavior corrections and
consumer-visible restrictions belong to the independently assessed minor
candidate. Keep the patch branch separate from `main`; do not merge a narrow
released-line tree over the ongoing sweep or transfer newer route proof into
the patch.

Each candidate still requires its exact source identity, API and semantic
compatibility assessment, normal consumer evidence, dependency/security
review, Queue-owned milestone QA and qualifying hosted CI. This direction
authorizes candidate preparation, not a tag, publication, consumer repin or
live proof. Tag creation and push still require Tom to name the final SHA.
The minor version is determined by its reviewed change; `v0.6.0` is a planning
target, not a prepared or authorized tag.

## Hosted Gate Delegation

`lint`, `lint:no-features`, `test`, and `floor` are satisfied for a candidate by
a green hosted `CI` run triggered by `workflow_dispatch` (or a push to `main`)
at the SHA to tag or at a commit with an identical tree; pull-request runs do
not qualify because the MSRV floor skips its tests there. `floor` is the
pinned-MSRV clippy and test pass already represented by that board; repeating
it locally re-runs a heavy gate the hosted run already proved.

A hosted run at another SHA does not count unless that commit's tree is
identical. If merge produces a new SHA whose tree is not identical to a
green qualifying run, dispatch `CI` with `workflow_dispatch` at the merge
SHA and wait for green before the tag request.

Local prepare either runs those four gates or records the qualifying run
id. The release note names the run.

Cheap gates still run locally, in this order: `fmt`, `qa`, `docs`,
`metadata`, `api`, `security`, `source`. A docs-index failure must fail
before clippy and the workspace tests.

Effigy v0.12.1 preserves `[release.gates]` declaration order and cannot skip
a configured gate from hosted evidence. Until Effigy grows that skip, the
default table omits the four delegated gates. Operators invoke the
local-heavy profile in `config/release.toml` only when no qualifying hosted
run exists. Native skip remains an Effigy Chatterbox request.

The lane, expected clocks, actors, and tag-request template live in
[the release playbook](release.md). The tag request goes
to the operator the moment those gates are green. Consumer smoke runs on the
tag afterwards and does not hold the tag.

## Release Authority

No manifest version, passing gate, changelog, clean commit, or generated
candidate grants authority to mutate external state.

Tom authorized exact `v0.5.3` annotated-tag creation and push on 2026-10-09:
“Release approved” (decision `fa46cc92-0466-4984-843f-aa5cefbbcda8`). Current
tagged identity is `v0.5.3` on `release/v0.5` at
`fa5ecfd8304030f58e447fd410382aee4056396b`, tree
`0597c27bf9b86a7e1c6bcb3f4fafa22a462303da`, tag object
`13862f5ffe0c9b75b9872865ee347c181dfc276e`. Its source-only annotation
records the registered-tool courier approval-wait correction with unchanged
SDK/native/Node and wire pins. Qualifying hosted workflow-dispatch run
`37854419710` and Queue milestone QA
`35c6ef9b-394e-4aab-bdbe-1122b8388b9b` passed on the identical-tree reviewed
candidate. No registry publication or GitHub Release is included. Working
application adoption and live proof remain consumer-owned and separately gated.

Tom authorized exact `v0.5.2` annotated-tag creation and push on 2026-10-08:
“Go for it” (decision `f1e95c52-063b-4362-a562-8e5e6a162b49`). Immutable
tagged identity is `v0.5.2` on `release/v0.5` at
`b83db0bdca4292e0d21775b9c0dc8b80ec05d003`, tree
`949d9ef1199cd21c188959dcb2c9e9bc5f2086ec`, tag object
`4d54ed92ec2dcdf44ebce019463d124996267810`. Its source-only annotation
records SDK registered-tool lease and per-turn usage corrections while
preserving released SDK/native/Node pins and wire-v1 compatibility.
No registry publication or GitHub Release is included.

Immutable `v0.5.1` remains at
`e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27`, tree
`d375b3227985e8e552ba9346d8f9b8631936db5f`, tag object
`97a6933abe13b2e8f05441e1ab950962e0683e65`, tagged 2026-09-13. Immutable
`v0.5.0` remains at `582d01d6b6890eed5195a1fcbee0ae985304a6c7`, tag object
`c772c5839806b6cbf1c9d3b495049b362e4c0c52`, tagged 2026-09-10. Immutable
`v0.4.4` remains at `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`, tag object
`41da6c1afe60380d248275bd0780c2e8f79e5ab9`, tagged 2026-09-09. Immutable
`v0.4.3` remains at `cbd4ddc8f9d6aa55bd947b92a55ea3a779582b79`, tag object
`d83004302801222258c3496791de1e3305571860`, tagged 2026-09-06. Immutable
`v0.4.2` remains at
`f94dd16f2e4db79c5b7c4440cc1eb2d20f8b6af8`, tag object
`927d14eccc16b5f24fc427913e087cd47fcfa499`, tagged 2026-09-06. Immutable
`v0.4.1` remains at `c3cce7504ffd5eae138a0190f1cd81332db68c3c`, tagged
2026-09-05. Immutable `v0.4.0` remains at
`56f3913ac99af44b6ff45384cfc53a0adea587ba`. Immutable
`v0.3.3` remains at `51d186208e75dca4c04f077dd7179ec3c2fafae9`. Later candidates require a
strictly greater version, an absent matching tag, and explicit operator
authorization of:

- source commit
- canonical branch and remote
- the selected later tag name
- annotated tag message
- confirmation that no crate publication or GitHub Release is included

A passing gate, changelog, or closeout commit does not authorize candidate
preparation, tag creation, or push. Creating the local tag and pushing it are
separate mutations unless one approval names both. Branch push, workflow
edit, crates.io publication, GitHub Release creation, consumer edits, and
provider work remain separate. Do not move or recreate an existing tag.

## Acceptance

- all 40 `v0.3.3` packages are separately consumable from one exact source
  identity
- OpenHands exists as a package without a production route
- the 29th and 30th Command Code and idioms packages first appear in `v0.3.2`
- the breaking binding-helper migration is explicit and limited to Codex and
  Ollama callers
- `publish = false` prevents accidental registry publication
- internal dependency direction is exact
- package compatibility and provider-interface versions remain separate
- the Rust floor and Apple Silicon support are explicit and tested
- source contents are clean, bounded, redacted, and reproducible
- public API is reviewed, semantically baselined, and documented
- dependency and security policy passes
- deterministic QA and external source-consumer proof pass
- an accepted working-application smoke remains recorded
- release notes and consumer instructions match the tagged source
- tag creation and push remain explicitly authorized external mutations
