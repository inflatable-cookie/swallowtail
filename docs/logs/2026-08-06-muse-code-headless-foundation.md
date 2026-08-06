# Muse Code Headless Foundation

Date: 2026-08-06
Roadmap: g03.045
Card: 138

## Outcome

Swallowtail current source now contains one separately selectable Muse Code
adapter and one exact `muse-code.headless` route. It binds the signed
`muse-bin-0.1.0-R708.1` payload, provider-owned local Meta account state,
explicit `meta` / `muse-spark-1.2` selection, all seven observed reasoning
efforts, read-only workspace authority, provider sandboxing, bounded event
projection, cancellation, deadlines, and joined cleanup.

The public guide, prepared example, route and feature matrices, configured
instance truth, failure posture, recovery absence, architecture, and package
contract now agree. Current source has 28 packages and 34 production routes.
Immutable `v0.1.0` and `v0.1.1` evidence remains 27 packages and 33 routes.
Muse has a separate unreleased semantic API baseline; no tagged baseline was
rewritten.

## Live Finding

The installed probe first exposed a logical executable-reference mismatch in
the test host. Binding the approved process under its exact versioned payload
identity fixed the probe without relaxing discovery.

The authenticated run then exposed one legitimate
`session.workspace_branch.observed` record after the provider terminal. The
parser now accepts bounded unknown records after terminal and projects them as
identity/lifecycle-only namespaced activity. Known run, task, output, model,
or duplicate terminal records after terminal still fail closed. The sanitized
Meta corpus freezes the observed ordering.

## Validation

- `python3 scripts/check-muse-code-corpus.py` — 5 passed
- `effigy validate:focused swallowtail-adapter-muse` — 20 passed across two
  binaries; warnings denied
- `effigy package:verify-affected swallowtail-adapter-muse` — extracted package
  passed
- `effigy package:metadata` — 28 current packages; immutable baseline 27
- `effigy package:api` — 27 immutable package APIs plus one unreleased Muse API
- `effigy package:docs` and `effigy check:examples` — passed
- `effigy qa:guides` — 34 routes, 23 route guides, 33 examples
- `effigy qa:routes` — 34 routes, 27 solutions, 67 activity operations
- `effigy qa:docs` — passed; tagged/current front-door distinction passed
- `effigy probe:muse-installed` — exact installed payload passed
- operator-gated `effigy probe:muse-spark-low` — two diagnostic invocations
  reached provider terminal output before the post-terminal projection fix;
  the final prepared Meta/Spark/low run passed

All three authenticated invocations ran with writes, shell, web tools, foreign
personal context, and session logging disabled. No login, logout, workspace
mutation, version bump, tag, GitHub Release, or registry mutation ran.

Source remains an uncommitted worktree on `main` atop
`7a614732412b0b7dc93b4f83a98badd23b4f24d5`.

## Next

Hold at the g03 evidence gate. Reassess Muse retained-session continuation and
recovery or the direct Meta Model API only under a separate operator-selected
roadmap.
