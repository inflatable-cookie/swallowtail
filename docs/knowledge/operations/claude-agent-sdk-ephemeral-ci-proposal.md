# SDK administrator-fixture CI proposal

Environment-setup authority request for task 234, Q-009. **Not activated.**
The planner's 2026-11-13 direction authorizes this proposal and owned-scratch
rehearsals only. No workflow, canonical host path or privilege is changed here.

## Requested boundary

Authorize one separately reviewed, manually dispatched fixture-only workflow
at its exact resulting source SHA: six fresh GitHub-hosted jobs, one attempt
per OS/state, five minutes per job, no automatic retry. Use `ubuntu-latest`
and `macos-latest`, preserving the existing ordinary CI jobs. Grant passwordless
`sudo -n` only for exclusive fixture creation and identity-checked cleanup in
these disposable jobs. All reads and hostile write attempts run as the normal
runner uid. No new tools, OS policy, namespace configuration, VM management,
SDK/native download, provider call or original execution is requested.

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

## Exact proposed job addition

The [reviewable job diff](claude-agent-sdk-ephemeral-ci.patch) adds only a new
`.github/workflows/sdk234-admin-fixture.yml`. It is retained as proposal data;
this task does not apply it. It contains complete preflight, root setup,
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
test checks patch applicability, Bash syntax, embedded Python compilation and
Node reader syntax without applying the patch or running its commands.

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

Before any privileged activation, return the exact applied workflow/source SHA,
adapter/reader/helper digests and this six-job scope for operator authority.
The current request authorizes preparing/reviewing that environment change,
not original/native execution. A further original-artifact request still needs
canonical injection proof, exact tuple/runtime custody and Nucleus admission.

References: [vendor settings](https://code.claude.com/docs/en/settings),
[managed settings](https://code.claude.com/docs/en/permissions#managed-settings),
[GitHub-hosted administrative privileges](https://docs.github.com/en/actions/reference/runners/github-hosted-runners#administrative-privileges).
