# 039 Pi RPC 0.84.4 Identity

Status: completed
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../015-pi-rpc-0-84-4-useful-newer.md`
Depends on: Contract 029; Research 267; official stable `0.84.4`

## Goal

Freeze exact official Pi RPC `0.84.4` identity and classify its selected
strict-LF RPC surfaces without changing a claim.

## Scope

1. Recheck npm, GitHub tag/commit, tarball, extracted dist, and selected
   git blobs. Do not infer identity from registry `latest` alone.
2. Keep host `0.83.0` observation-only. Do not install or update it.
3. Compare selected RPC types, framing, session-cwd, argv, and extracted
   dist with frozen `0.84.3`.
4. Classify changelog extras as unmapped unless they change a selected
   mapped flag or command.
5. Audit `pi.sdk-sidecar` exact `0.84.2`. Do not raise that pin.
6. Add Research 268 and one secret-free `0.84.4` identity/protocol corpus.
7. Commit identity evidence before any selection, matrix, guide, changelog,
   or standing-lane claim edit.
8. Record compatible extension, private milestone, new revision, or stop.

## Out Of Scope

Production claim edits, sidecar pin change, Oh My Pi, Gemini, another
family, provider contact, login, install, host update, live probe,
projection, skill, papercut, g05.009 card 034, release, or execution of
downloaded official binaries.

## Acceptance Criteria

- official identity is corroborated through independent official channels
- mapped and material unmapped additions are explicit
- current production claims are byte-for-byte unchanged in this commit
- fixture provenance, digests, and negative boundaries are load-bearing
- sidecar exact `0.84.2` is not raised
- card 040 continues only for an admitted segment

## Validation

- `effigy validate:focused swallowtail-adapter-pi`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, to card 040 only after an admitted segment is recorded.

## Result

Official stable remained exact `0.84.4`. Host `0.83.0` matches the previously
frozen `0.84.2` host digest. Research 268 and the frozen corpus landed in
identity-only commit `60e7a0a0`; production claims remained unchanged in that
commit. Mapped `jsonl.ts`, `session-cwd.ts`, `json-event.ts`, and `args.ts`
are byte-identical to `0.84.3`. Unused `clear_queue` stays unmapped.
`pi.sdk-sidecar` stays exact `0.84.2`. The selected protocol classifies as a
compatible extension of `pi.rpc.strict-lf-v0.84.0-message-update-delta`, so
card 040 continued.
