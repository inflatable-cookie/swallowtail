# SDK administrator-fixture CI proposal

Fixture-only environment record for task 234, Q-009. **One attempt consumed.**
Tom's decision `32d31615-702b-4e3b-a972-56b4fb18bae2` v2 authorized preparation.
Decision `5912d4df-9a3c-4bfe-950e-8b2dad638bb2` v2 on 2026-11-13 at
11:41:31.965Z separately answered "Authorize one fixture attempt." That attempt
completed below. No replay, original/native execution or qualification follows.

## Approved boundary

One manually dispatched attempt covered six fresh GitHub-hosted jobs: one per
OS/state, five minutes per job, no automatic retry. Ubuntu/macOS ordinary CI
stayed separate. Privilege covered exclusive fixture creation and checked cleanup
on those disposable hosts; canonical reads and hostile write attempts used the
normal runner uid. No new tools, OS policy, namespace configuration, VM management,
SDK/native download, provider call or original execution was authorized.

The existing Node setup step is copied exactly. Refuse a runtime other than
Node 22.23.2 or the observed per-platform digest; do not chase an updated image.
Record actual OS, kernel, architecture, uid, Node/Python executable paths and
digests, source SHA, helper digest and fixture receipt in the job log.
Runner labels and `setup-node` requests alone prove nothing.

| State | Canonical policy file | Opposite HOME/project fixtures |
| --- | --- | --- |
| `absent` | No file; the newly owned policy directory still exists | `denyInsteadOfPrompting: true` |
| `disabled` | `{"permissions":{"denyInsteadOfPrompting":false}}` | `denyInsteadOfPrompting: true` |
| `managed-deny` | `{"permissions":{"denyInsteadOfPrompting":true}}` | `denyInsteadOfPrompting: false` |

Linux canonical file: `/etc/claude-code/managed-settings.json`.
Darwin canonical file: `/Library/Application Support/ClaudeCode/managed-settings.json`.
The policy directory must not already exist, even if empty. Never adopt,
overwrite, rename, back up or restore an existing administrator configuration.
Parent components must be real directories owned by root without group/other
write permission. Policy directory: root-owned 0755; policy: root-owned 0444,
one link; lease: root-owned 0600, one link. The lease binds run/attempt/job/state,
directory device/inode and exact policy bytes. No ACL or ownership relaxation.

## Applied job and activation gate

The [reviewable job diff](claude-agent-sdk-ephemeral-ci.patch) adds only a new
`.github/workflows/sdk234-admin-fixture.yml`. The [applied workflow](../../../.github/workflows/sdk234-admin-fixture.yml)
matches that diff byte for byte. The approved proposal was retained at source
`6bb1ca20130f98a8af2ef2dd3fc03e6640f84a9e`, manifest SHA-256
`cf370ed497f9f80e6ba7cdb865252f6c9556f55d1443b9f8cba039d66e4f4f52`;
the unchanged diff SHA-256 is
`16a02b1e0143992963b7a53178349bba0c4d44839f68df4a513622884da25bc7`.
The current manifest follows the updated test, selectors and owning document;
it does not replace that historical approval identity. The workflow contains complete preflight, root setup,
unprivileged read and always-run cleanup commands, with no omitted adapters.
No existing CI job is changed. No pull-request, push, schedule or
`pull_request_target` trigger is proposed.

The operator must name the exact reviewed workflow commit, dispatch source
SHA and [proposal manifest](claude-agent-sdk-ephemeral-ci.sha256) digest before
activation. Source input accepts exactly 40 lowercase hex characters;
checkout uses that SHA with persisted credentials disabled. The job verifies
HEAD, a clean tree, the externally supplied manifest digest and every bound
file before measuring runtimes. It also compares the applied workflow bytes
to the bound draft. The manifest binds the new helper, test, selectors, proposal
and patch plus the unchanged confinement execution record, without a hash cycle.
Both Node version and digest
must match the retained platform observations. Python's actual executable and
digest are recorded; that measurement supplies no custody ruling.

The root phase loads exactly the helper bytes it hashes before compilation.
Its reviewed adapter rejects non-root execution, checks every canonical parent
component for root ownership, symlink absence and denied group/other writes,
and calls only `prepare` or `cleanup`. It never calls a shell, provider, SDK,
native executable or a package installer. The independent Node reader runs with
`env -i`, explicit HOME/workspace/TMP and no startup overrides. It reads the
literal canonical path, asserts the state, then proves policy/new-file writes
are denied to the runner uid. No native settings-path override is used.

The draft stages its adapter and reader from literal reviewed workflow bytes
into owned scratch. `$GITHUB_ENV` is only a runner handoff, not authority or
custody. Applying the draft requires binding those literals, the helper,
workflow and exact source SHA to the environment-setup authority. This is not
a supported Nucleus original-code proof declaration.

Setup refusal, read failure, cleanup refusal and missing observations each
fail the job. A pre-existing directory is preserved and cleanup refuses its
missing lease. A partially created or changed fixture is preserved for the
ephemeral host's disposal; no privileged repair/retry is proposed. Job timeout
or cancellation can prevent cleanup, so the authority must cover discarding
that whole ephemeral job, never reusing it for a consumer/native proof.

## Proof limits and next gate

`validate:claude-agent-sdk-admin-fixture` rehearses the helper under a freshly
owned scratch parent, with the real current uid as expected owner. It covers
both directory spellings, all three states, exclusive creation, mode/owner
refusal, altered bytes, symlinks/hardlinks, replaced directory, unexpected
entries, foreign lease and cleanup that preserves an owned sibling. It never
claims root ownership, canonical access, native precedence or platform support.
Compile and check-only format routes cover both new Python files. A separate
test checks exact approved/applied byte equality, reverse patch applicability,
Bash syntax, embedded Python compilation and Node reader syntax without
running its commands. A routing mutation test accepts only this exact non-Cargo
workflow and preserves the ordinary Cargo guard for every other workflow.
`check:claude-agent-sdk-admin-workflow` runs installed actionlint on the actual
workflow. `check:oxide-routing` compiles the changed routing guard.

The six proposed jobs would establish canonical fixture readability and
ordinary-uid write denial only. Opposite user/project files are deliberate
inputs; this independent reader does not emulate native settings discovery.
No SDK `managedSettings`, settings-path environment override or `--managed-settings`
substitutes for the canonical path. No settings daemon, MDM, registry, server
managed policy or real user file is accessed as a diagnostic.

Source/runtime measurement and verified helper bytes do not settle hostile
same-user interpreter, Python library, Node or workflow custody. Loading the
adapter from runner-owned scratch is an explicit trust assumption for this
fixture-only proposal, not an accepted original-code boundary. Custody still
needs its own ruling and immutable execution route. The frozen SDK/native
artifact ledger is unchanged and no artifact enters these jobs. Original
activation remains unconditionally refused; the production pair stays
0.3.317/2.1.317.

## Consumed immutable-source attempt

Registration235 placed the exact workflow on main
`1cbb29f459950a76fb9104fb10947fd87694e483`, workflow id `417208829`.
Immediate preflight verified zero earlier dispatches, the exact queue ref,
checked-out and remote workflow bytes, and the execution manifest.
[Run 43420542403](https://github.com/inflatable-cookie/swallowtail/actions/runs/43420542403)
completed successfully, attempt 1, at execution source
`bfd9341bde002eef64beb5560f2bfeb51d6b06a6`. Its manifest was
`eceeb513d7adb0ad3a8889f2f39686dd64a23855f9490a401c21913b60f702e6`,
not the earlier preparation manifest. Later documentation and main reconciliation
do not transfer this authority to another SHA.

All six logs recorded root-owned setup receipts, ordinary-uid canonical reads,
expected false/false/true outcomes and successful identity-checked cleanup.
The helper returned after removing only its checked policy/lease/directory;
each independent GitHub cleanup step also concluded successfully. No separate
post-cleanup filesystem inspection or original native discovery is claimed.

| Platform/state | Job | Observed kernel | Effective denial |
| --- | --- | --- | --- |
| Linux absent | [131869140709](https://github.com/inflatable-cookie/swallowtail/actions/runs/43420542403/job/131869140709) | 7.0.0-1016-aws, x86_64 | false |
| Linux disabled | [131869140749](https://github.com/inflatable-cookie/swallowtail/actions/runs/43420542403/job/131869140749) | 7.0.0-1019-azure, x86_64 | false |
| Linux managed-deny | [131869140697](https://github.com/inflatable-cookie/swallowtail/actions/runs/43420542403/job/131869140697) | 7.0.0-1019-azure, x86_64 | true |
| Darwin absent | [131869140730](https://github.com/inflatable-cookie/swallowtail/actions/runs/43420542403/job/131869140730) | 25.6.0, arm64 | false |
| Darwin disabled | [131869140407](https://github.com/inflatable-cookie/swallowtail/actions/runs/43420542403/job/131869140407) | 25.6.0, arm64 | false |
| Darwin managed-deny | [131869140777](https://github.com/inflatable-cookie/swallowtail/actions/runs/43420542403/job/131869140777) | 25.6.0, arm64 | true |

Actual Node was 22.23.2 in every job. Linux Node SHA-256 was
`3517c2df0b2f8cd7f422b4b8450ef81c6889f08eb03e281d6de9079b15e6a327`;
Darwin was `18e387c90ab8a8400183e8bdd396376e1e875b91b4c874b894dcade7b35bf572`.
Observed Python executable SHA-256 was
`be9a2a5eada8c89c1c399fdfb8397179e877c20d8db0739c802f726d4d0e69fd`
on Linux and `0f407a0ac747337b659538c9560e4fdd48b354dbdbe0a143276b56ee23899fb7`
on Darwin. These are observations, not interpreter-custody admission.
Every reader reported `nativeAdministratorDiscovery: false` and
`originalActivation: false`.

The historical command is retained below as a consumed attempt, not a new grant:

```sh
gh workflow run sdk234-admin-fixture.yml \
  --repo inflatable-cookie/swallowtail \
  --ref ns-d7afab60-9a93-4414-95e7-61608a703ab7 \
  -f source_sha=bfd9341bde002eef64beb5560f2bfeb51d6b06a6 \
  -f proposal_sha=eceeb513d7adb0ad3a8889f2f39686dd64a23855f9490a401c21913b60f702e6
```

GitHub's [workflow_dispatch prerequisite](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#workflow_dispatch)
was supplied by235 before dispatch. Runtime/workflow custody stays separate from
this fixture-only branch-ref handoff. No further fixture attempt is granted.

A further original-artifact request still needs
canonical injection proof, exact tuple/runtime custody and Nucleus admission.

References: [vendor settings](https://code.claude.com/docs/en/settings),
[managed settings](https://code.claude.com/docs/en/permissions#managed-settings),
[GitHub-hosted administrative privileges](https://docs.github.com/en/actions/reference/runners/github-hosted-runners#administrative-privileges).
