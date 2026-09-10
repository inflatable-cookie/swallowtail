# g05.050 v0.5.0 Tagged-Source Consumer Proof

Status: completed; remote-tag consumer resolved every selected Swallowtail package at exact peel `582d01d6`; registered-only Claude SDK binding compiled with explicit `ReadWrite` on MSRV `1.95.0`; provider-free; documentation closeout PR pending review
Owner: Tom
Created: 2026-09-10
Depends on: g05.049; Contract 036; annotated `v0.5.0`
Vision tags: source release, consumer proof, Claude route

## Outcome

Prove that an isolated external Cargo consumer can select Swallowtail through
the immutable remote `v0.5.0` tag, resolve every selected package to the exact
tag peel, and compile the registered-only Claude SDK session surface needed by
Desktop g02.051. Publish one source-linked capsule for the Desktop coordinator.

This is provider-free release evidence. It does not mutate a consumer, contact
Claude, or claim that Desktop's working route has passed.

## Ready-State Rubric

- [x] Annotated tag `v0.5.0` exists locally and remotely as object
      `c772c5839806b6cbf1c9d3b495049b362e4c0c52`.
- [x] The tag peels to candidate
      `582d01d6b6890eed5195a1fcbee0ae985304a6c7`, tree
      `eebb6978be365f79ae41436912240886d0aecf78`.
- [x] Tag-triggered CI run 34467974791 passed all 11 jobs at that candidate.
- [x] g05.046 supplies the registered-only profile with zero native tools and
      explicit `Read` or `ReadWrite` access.
- [x] Desktop task `1e7b9baf-9b97-4e49-a6d4-17aa24bbcb5c` remains blocked
      before edits or provider contact on its immutable `v0.4.4` handoff.
- [x] Tom said `Continue` after Chatterbox proposed this proof followed by
      resumption of the preserved Desktop task.

## Dispatch Manifest

- **State:** ready; dispatch after this planning commit reaches pushed `main`.
- **Completion:** verify the tag capsule; run the repository source-consumer
  selector from a clean detached checkout of the tag; build one temporary
  external Cargo consumer whose direct Swallowtail dependencies use the
  canonical Git URL and `tag = "v0.5.0"`; exercise the public registered-only
  Claude SDK preparation surface at compile time; prove every selected
  Swallowtail package has a Git source ending in the exact peeled SHA; retain
  command output and SHA-256 receipt; publish one log and closeout.
- **Owned mutable paths:** this task; one new g05.050 log;
  `docs/logs/README.md`; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`,
  `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`.
- **Worker:** release-evidence worker; no credentials or provider access.
- **Excluded:** crates, tests, manifests, lockfile, scripts, release baselines,
  contracts, release notes, tags, branches, workflows, provider calls,
  credentials, Desktop or other consumer mutation, dependency repin, live
  application smoke, GitHub Release, registry, binary, sidecar, installer, or
  model publication.
- **Serial edge:** only an accepted g05.050 closeout may be relayed into the
  separate Desktop planning amendment and same-task resume. No Desktop work
  runs concurrently with this proof.
- **Escalation:** a tag/object/peel mismatch, non-tag dependency source, compile
  failure, or inability to express the registered-only surface stops here.
  Never repair an immutable tag or widen into consumer implementation.

## Work

1. Fetch the canonical tag without mutation. Verify local and remote tag
   object equality, annotated type, exact peel and tree, and the approved
   annotation bytes. Verify the detached worktree is clean and at the peel.
2. From that clean detached checkout run `effigy package:source-consumer`.
   Record the exact command output and retain a SHA-256 digest of the complete
   transcript.
3. In a fresh temporary directory outside the repository, create a minimal
   Cargo application. Select only the Swallowtail packages needed to compile
   the public registered-only Claude SDK session preparation path; every direct
   Swallowtail dependency must use
   `git = "https://github.com/inflatable-cookie/swallowtail"` and
   `tag = "v0.5.0"`. No path, revision, branch, patch, or workspace override is
   allowed.
4. Generate a lockfile, compile with the repository MSRV, and inspect locked
   metadata. Fail unless every selected Swallowtail package has a Git source
   ending in
   `#582d01d6b6890eed5195a1fcbee0ae985304a6c7`; fail on any local path source or
   mixed source identity. The compile must instantiate or type-check the
   registered-only binding with zero native tools and explicit `ReadWrite`,
   without opening a session or contacting a provider.
5. Remove the temporary consumer and detached worktree. Confirm canonical
   Swallowtail remains clean and the tag unchanged. Commit only the evidence
   log and closeout surfaces, open one PR, obtain independent exact-head review,
   merge, and synchronize canonical `main`.

## Result

Closed 2026-09-10. The tag capsule was verified unchanged: annotated tag
object `c772c5839806b6cbf1c9d3b495049b362e4c0c52` identical locally and on the
canonical remote, peeling to `582d01d6b6890eed5195a1fcbee0ae985304a6c7`
(tree `eebb6978be365f79ae41436912240886d0aecf78`) with the approved
annotation digests `0949222c…` (object bytes) and `eb99cdf8…` (body).

From a clean detached checkout at that peel, `effigy package:source-consumer`
passed at exit 0 with transcript digest `2d198fe1…`. In a fresh temporary
directory outside the repository, one minimal Cargo application declared the
three direct Swallowtail dependencies needed by the registered-only Claude
SDK surface (`swallowtail-adapter-claude-agent`, `swallowtail-runtime`,
`swallowtail-core`), each through the canonical HTTPS Git URL with only
`tag = "v0.5.0"` — no path, branch, `rev`, patch, or workspace override.
`cargo +1.95.0 generate-lockfile` locked 47 packages, and locked metadata
proved all six reachable Swallowtail packages (adapter-claude-agent, core,
host-local, idioms, protocol-acp, runtime) carry the one identical source
`git+https://github.com/inflatable-cookie/swallowtail?tag=v0.5.0#582d01d6b6890eed5195a1fcbee0ae985304a6c7`
with zero `file://` or path sources. `cargo +1.95.0 check --locked` finished
cleanly at transcript digest `567c946e…`, type-checking
`ClaudeAgentSdkRegisteredOnlyBinding::new` with `ResourceAccess::ReadWrite`
and `ClaudeAgentSdkPermissionMode::AcceptEdits`, plus a compiled
`is_registered_only()` and explicit-access check through the public API. No
binding was constructed with a live preparation, no session opened, and no
provider was contacted. Manifest, lockfile, and source digests are recorded
in the evidence log. Cleanup removed the detached worktree and consumer; the
tag and canonical Swallowtail were re-verified unchanged afterward.

## Review Oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| The selector is the remote immutable tag | local path, branch, `rev`, patch, or moving `main` | temporary manifest uses canonical Git URL plus only `tag = "v0.5.0"` |
| All packages share one source | one transitive Swallowtail package leaks to a sibling path or other revision | metadata enumerates every selected Swallowtail package and every source ends in the exact peel |
| The needed API is consumable | compile only unrelated core/Codex types | source compiles the public registered-only Claude SDK binding with zero native tools and explicit `ReadWrite` |
| Proof is provider-free | session open, credential lookup, sidecar launch, or prompt | compile/check only; no process or network use beyond Cargo/Git source retrieval |
| Tag remains immutable | move, recreate, force, or edit release source | tag object, peel, tree, and message agree before and after; no tag mutation |
| Desktop stays separate | edit or repin Desktop while proving source consumption | Swallowtail documentation-only PR; Desktop task remains blocked until relay |

## Stop Conditions

- Stop on dirty source, tag disagreement, wrong object type, peel, tree, or
  annotation.
- Stop if Cargo does not honor the exact remote tag for every selected package,
  the registered-only public surface does not compile, or MSRV compilation
  fails.
- Stop before source, script, manifest, release-note, tag, workflow, provider,
  credential, or consumer-repository mutation.
- Do not substitute the candidate SHA for the tag selector to make the proof
  pass.

## Evidence

Record tag object, peel, tree, annotation digest, detached-checkout identity,
repository source-consumer output, remote-tag consumer manifest digest,
lockfile/metadata source identities, compilation command and result, complete
transcript SHA-256, cleanup, exact PR/head/review/merge/closeout, zero provider
contact, no tag or consumer mutation, and retained non-claims.

## Next Task

The proof closed 2026-09-10. Chatterbox relays the exact capsule (tag
object/peel/tree, both consumer proofs, every package source identity, and
transcript digests in [the evidence log](../../logs/2026-09-10-g05-050-v0-5-0-tagged-source-consumer-proof.md))
into the Desktop planning amendment, promotes the narrow `v0.5.0` pin
amendment required by the existing g02.051 handoff, and resumes task
`1e7b9baf-9b97-4e49-a6d4-17aa24bbcb5c` in its preserved workspace. No provider
call follows from this Swallowtail task.
