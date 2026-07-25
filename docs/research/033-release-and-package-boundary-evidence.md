# 033 Release And Package Boundary Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-24

## Question

What is the smallest honest release boundary for Swallowtail's current
23-crate workspace, and which package, version, MSRV, registry, and release
authority choices still require operator approval?

## Method

Evidence was accessed 2026-07-24.

- inventoried all workspace manifests, targets, features, direct dependencies,
  crate-root exports, package file lists, fixtures, and generated package files
- resolved the current dependency graph and recorded declared transitive Rust
  version floors
- inspected local and remote tags, GitHub releases, changelog state, Effigy
  tasks, release automation, and registry configuration
- inspected Nucleus and Soundcheck manifests, locks, and toolchain files
  read-only
- queried the public crates.io API for every exact package name
- revalidated current official Cargo packaging, publishing, workspace,
  dependency, SemVer, and Rust-version documentation
- checked the current official Rust release line

No manifest, consumer repository, registry, credential, tag, workflow, or
release was changed. `cargo package --list` only listed candidate contents. No
package archive was built and no publish command was run.

## Workspace Baseline

The workspace has 23 library crates. Every crate:

- inherits version `0.1.0`, edition 2024, MIT license, and the repository URL
- has a public library target and no binary, example, or build-script target
- has no Cargo feature
- has no `rust-version`
- has no explicit `publish` restriction
- uses path-only requirements for internal dependencies

Twenty-two crates have descriptions.
`swallowtail-transport-acp-remote` does not. No package declares a readme,
homepage, keywords, or categories.

The virtual workspace explicitly uses resolver 2. Edition 2024 does not change
the resolver of a virtual workspace automatically. A future Rust-version-aware
release policy therefore needs an explicit resolver decision.

The crate-root export count below is an inventory marker, not a SemVer API
baseline. It counts top-level `pub` declarations in each `src/lib.rs`.
Package files are the current `cargo package --list --allow-dirty` result,
including Cargo-generated manifest and VCS records.

| Crate | Candidate role | Normal internal dependencies | Root exports | Package files | Current dependency floor |
| --- | --- | --- | ---: | ---: | --- |
| `swallowtail-core` | public foundation | none | 27 | 60 | 1.85 edition floor |
| `swallowtail-runtime` | public foundation | core | 42 | 67 | 1.85 |
| `swallowtail-host-local` | public host support | core, runtime | 2 | 30 | 1.85 |
| `swallowtail-testkit` | public adapter support | core, runtime | 18 | 75 | 1.85 |
| `swallowtail-protocol-acp` | public protocol | none | 14 | 86 | 1.85 |
| `swallowtail-protocol-openai-chat` | public protocol | none | 10 | 22 | 1.85 |
| `swallowtail-transport-acp-remote` | public transport | core, runtime, ACP | 4 | 24 | 1.88 |
| `swallowtail-adapter-alibaba-model-studio` | public opt-in adapter | core, runtime | 4 | 50 | 1.86 |
| `swallowtail-adapter-anthropic` | public opt-in adapter | core, runtime | 2 | 91 | 1.86 |
| `swallowtail-adapter-bedrock` | public opt-in adapter | core, runtime | 10 | 41 | 1.94.1 |
| `swallowtail-adapter-claude-agent` | public opt-in adapter | core, runtime, ACP | 2 | 31 | 1.85 |
| `swallowtail-adapter-codex` | public opt-in adapter | core, runtime | 4 | 54 | 1.85 |
| `swallowtail-adapter-deepseek` | public opt-in adapter | core, runtime, OpenAI chat | 2 | 55 | 1.86 |
| `swallowtail-adapter-gemini` | public opt-in adapter | core, runtime, ACP | 2 | 67 | 1.86 |
| `swallowtail-adapter-kimi` | public opt-in adapter | core, runtime, ACP | 2 | 36 | 1.85 |
| `swallowtail-adapter-kimi-platform` | public opt-in adapter | core, runtime, OpenAI chat | 1 | 22 | 1.86 |
| `swallowtail-adapter-llama-cpp` | public opt-in adapter | core, runtime, OpenAI chat | 1 | 53 | 1.86 |
| `swallowtail-adapter-opencode` | public opt-in adapter | core, runtime | 2 | 56 | 1.86 |
| `swallowtail-adapter-ollama` | public opt-in adapter | core, runtime | 3 | 51 | 1.86 |
| `swallowtail-adapter-pi` | public opt-in adapter | core, runtime | 3 | 48 | 1.85 |
| `swallowtail-adapter-openai` | public opt-in adapter | core, runtime | 2 | 87 | 1.86 |
| `swallowtail-adapter-qwen` | public opt-in adapter | core, runtime | 3 | 30 | 1.85 |
| `swallowtail-adapter-xai` | public opt-in adapter | core, runtime | 3 | 33 | 1.85 |

All 23 crates are consumer-usable libraries. None is an internal command,
release tool, generated package, or umbrella facade. The only plausible
intentionally unpublished role is an operator choice to withhold a public
adapter, not a role implied by its implementation.

## Exact Dependency And Publication Order

Normal internal dependency edges are:

1. `swallowtail-runtime` to core
2. host-local and testkit to core and runtime
3. ACP remote transport to core, runtime, and ACP
4. every adapter to core and runtime
5. Claude Agent, Gemini, and Kimi adapters additionally to ACP
6. DeepSeek, Kimi Platform, and llama.cpp adapters additionally to the OpenAI
   chat protocol

Path-only internal development dependencies do not constrain registry
publication, but they remain relevant to packaged-test reproducibility.

The exact candidate publication stages are:

1. `swallowtail-core`, `swallowtail-protocol-acp`,
   `swallowtail-protocol-openai-chat`
2. `swallowtail-runtime`
3. host-local, testkit, ACP remote transport, and all adapters

Crates within one stage have no normal internal edge between them. Actual
registry publication must wait for each prior stage to appear in the registry
index. This is a publication order, not authority to publish.

Cargo rejects a publishable normal or build path dependency without a version
key. It ignores the path when resolving a published package. The future
manifests therefore need both the local path and a registry version
requirement. Path-only development dependencies are permitted.

## Package Contents

The 23 package lists contain tests and 281 deterministic fixture files totalling
243,409 bytes. Cargo would also generate a normalized `Cargo.toml`,
`Cargo.toml.orig`, `Cargo.lock`, and `.cargo_vcs_info.json` for each package.

The bounded scan found:

- no private-key material
- no token-shaped `sk-` value
- no generated source or build script
- no example or binary target
- three source files containing `/Users/` only as negative redaction
  assertions
- two source files naming provider credential variables only as negative
  redaction assertions

This is enough to keep the deterministic fixtures in the candidate package
set. It is not a substitute for inspecting the final `.crate` archives after
versioned internal dependencies exist.

## Release And Registry State

- `CHANGELOG.md` says no release has been published.
- The local repository and `origin` have no tags.
- GitHub reports zero releases.
- Effigy has QA, documentation, test, and installed-probe tasks, but no release
  preparation or execution task.
- No Cargo registry default or package-specific registry is configured in the
  repository.
- The public crates.io API returned `404` for every exact package name.

The crates.io result means no matching public crate was visible at observation
time. It does not reserve a name. Crates.io allocates names first-come,
first-served, and package ownership begins through publication. Registry
account, token, owner, team, and final name authority remain unresolved.

## Consumer Evidence

Nucleus and Soundcheck both consume the same four crates:

- `swallowtail-core`
- `swallowtail-runtime`
- `swallowtail-host-local`
- `swallowtail-adapter-codex`

Both use sibling-checkout path dependencies. Their lock files resolve all four
to local `0.1.0` packages without a registry source or checksum.

Neither repository declares `rust-version` or a repository toolchain file.
Both default to edition 2021; one isolated Nucleus crate already uses edition
2024. The local Swallowtail development host has Rust and Cargo `1.96.0`.
Official Rust releases show `1.97.1` as current. Consumer evidence therefore
does not justify a latest-only MSRV and does not prove an older exact floor.

The first consumer handoff must replace path assumptions with one exact
packaged or published version and preserve a path override for rollback.
Neither consumer authorizes an edit during this lane.

## Current Cargo Rules

The official sources establish:

- `cargo package` assembles and verifies a distributable archive without
  uploading it
- `cargo publish --dry-run` performs publish checks without upload
- an actual publish is permanent, cannot overwrite a version, and requires
  registry authentication
- a normal or build path dependency needs a version before packaging
- Cargo's default `0.1.0` requirement means `>=0.1.0, <0.2.0`
- Cargo treats the left-most non-zero component as the compatibility boundary
- `rust-version` is a declared support promise and may affect dependency
  resolution
- raising `rust-version` is assumed to be a minor incompatibility
- workspace package metadata can inherit version, publish, readme, and
  Rust-version fields

Provider and harness interface ranges remain unrelated. A Swallowtail crate
release can change its qualified Codex or Kimi range without changing Cargo's
meaning of a compatible crate dependency, provided the Rust API remains
compatible.

## Package-Set Comparison

| Option | Strength | Cost |
| --- | --- | --- |
| Four current-consumer crates | smallest first upload; proves today's Codex consumers | leaves 19 documented public routes path-only and does not validate the library's stated breadth |
| Shared foundations only | establishes reusable vocabulary and adapter-author support | gives consumers no complete provider route |
| All 23 consumer-usable crates | matches the realized repository boundary; validates every public route; avoids an artificial private tier | larger metadata, ownership, package, and coordinated validation burden |

Recommendation: treat all 23 as public packages. Prepare and verify all 23 in
the first non-published release candidate. Publication remains a separate
human decision and must follow the three-stage dependency order.

## Version-Model Comparison

| Option | Strength | Cost |
| --- | --- | --- |
| Independent versions | adapter-only releases can move alone | 23 baselines, more dependency combinations, and little evidence that current tightly coupled public types are independently stable |
| Coordinated workspace version | one compatibility line, one tag, simple consumer guidance, and current manifest alignment | unchanged crates may receive coordinated version bumps |

Recommendation:

- keep one coordinated workspace version through pre-1.0
- use `0.1.0` as the first candidate
- use path plus ordinary compatible `0.1.0` internal requirements
- permit patch releases only for Cargo-compatible Rust API changes
- move any breaking public API change to the next workspace minor
- do not infer API 1.0 or provider-interface compatibility from the crate
  version

Exact `=0.1.0` internal requirements would force every dependent upload for
every patch and provide little value if patch releases obey their compatibility
promise. Independent versions can be reconsidered after actual release
pressure shows a stable package boundary.

## MSRV Comparison

Latest-only Rust is easy for maintainers but needlessly couples consumer
upgrades to the Swallowtail release date. Declaring the source minimum of each
crate would promise a broad matrix that current CI has not verified.

The official six-month release span is `1.93.0` through current `1.97.1`.
Current resolved normal dependency floors fit within `1.93`, except the
Bedrock SDK pins require `1.94.1`.

Recommendation:

- guarantee an N-4 stable-minor window at each new Swallowtail minor line
- declare `1.93` for the initial line across all packages except Bedrock
- declare `1.94.1` for `swallowtail-adapter-bedrock`
- use resolver 3 so dependency selection can respect declared Rust versions
- verify each declared floor plus current stable before the release candidate
- freeze the floor through compatible patch releases
- raise it only in a breaking pre-1.0 minor, or under an explicit
  dependency/security exception with changelog and consumer evidence

This is a support policy, not a claim that each crate cannot compile on an
older compiler. Other toolchains may work without being guaranteed.

## Release-Preparation Comparison

Immediate workflow automation would encode policy before the first package
graph is proven. Fully manual commands are difficult to reproduce.

Recommendation:

- add deterministic Effigy preparation and verification selectors
- keep those selectors credential-free and upload-free
- build and inspect packages in dependency order from a clean commit
- require a manually curated changelog and release record
- require one explicit human gate for registry upload, ownership, tag, push,
  and GitHub release
- do not let a workflow or version bump imply authority to publish

Initial verified target evidence is Apple Silicon macOS only. Other targets
may work and must not be hard-denied, but they remain unverified until a
separate target matrix exists.

## Recommended Decision Bundle

Card 002 can become mechanical if the operator approves:

1. all 23 crates are public, separately consumable packages
2. crates.io is the initial registry target
3. all crates begin at coordinated `0.1.0`
4. internal registry requirements use ordinary pre-1.0 compatible ranges
5. the initial MSRV is `1.93`, except Bedrock at `1.94.1`
6. resolver 3 and floor-plus-current testing enforce the MSRV promise
7. package preparation is deterministic through Effigy
8. every upload, owner change, tag, push, and GitHub release remains a separate
   human-approved mutation

No item is adopted by this research record alone.

## Unresolved Authority

- operator approval or amendment of the decision bundle
- crates.io account and owner/team identity
- release credential availability
- final name availability at publication time
- explicit publication and tag authorization after the non-published release
  candidate passes

## Primary Sources

- [Cargo package](https://doc.rust-lang.org/cargo/commands/cargo-package.html)
- [Cargo publish](https://doc.rust-lang.org/cargo/commands/cargo-publish.html)
- [Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Cargo dependency specification](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)
- [Cargo Rust-version policy](https://doc.rust-lang.org/cargo/reference/rust-version.html)
- [Rust release announcements](https://blog.rust-lang.org/releases/)
- [crates.io API](https://crates.io/data-access)
- [Swallowtail GitHub releases](https://github.com/inflatable-cookie/swallowtail/releases)

## Promotion Targets

- provisional Spec 004 for the operator decision
- roadmap g02.001 card 002 for architecture and contract promotion
- a new durable release contract
- repository architecture for the public package topology
- cards 003-004 for package proof and consumer handoffs

## Promotion Result

The operator approved the recommended decision bundle on 2026-07-24.

- accepted structure moved to
  `docs/architecture/release-and-package-topology.md`
- durable rules moved to Contract 036
- provisional Spec 004 moved to the archive
- roadmap g02.001 cards 003-004 now own realization and release-candidate
  evidence

Research remains evidence, not release authority.
