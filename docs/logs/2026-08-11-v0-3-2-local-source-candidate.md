# v0.3.2 Local Source Candidate

Date: 2026-08-11
Roadmap: `../roadmaps/g03/067-v0-3-2-source-patch-release.md`
Card: `../roadmaps/g03/batch-cards/211-v0-3-2-local-source-candidate.md`

## Result

The complete local `v0.3.2` source candidate is prepared. All 30 workspace
packages and coordinated internal requirements use `0.3.2`; the lock, promoted
changelog, 36-route inventory, semantic API baselines, release notes, examples,
and source-only distribution copy agree.

Command Code and idioms are separately selectable packages. Claude Code
response-only remains exact `2.1.227`, text-only, tool-free, and carries no JSON
or schema guarantee. Existing provider and consumer boundaries are unchanged.

## Validation

- `effigy release prepare --yes --check-gates --version 0.3.2` — prepared
- all 11 configured gates passed on the prepared tree
- 1,625 workspace tests passed; 17 were skipped
- isolated source consumer passed; its temporary validation commit is not a
  candidate source identity
- Effigy `v0.11.0+local.53a4971`, exact commit
  `53a4971da31344c0f1f3bb24308e78ee2e85ec3c`

## Authority

The candidate remains local and uncommitted on base HEAD
`d9b7bab93671f3fbd2b3166ed3d3de8a9f8a462d`. The validation snapshot is not a
canonical source identity. No commit, push, workflow, tag, registry, consumer,
or provider mutation ran.

## Next Move

After explicit operator acceptance, commit and push the exact candidate, then
require canonical CI at that SHA. Tagging remains a separate authorization.
