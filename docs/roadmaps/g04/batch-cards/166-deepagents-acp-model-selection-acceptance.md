# 166 Deep Agents ACP Model Selection Acceptance

Status: blocked
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Blocked by: Research 206 empty deliver-now set; card 165 not executed
Milestone: [g04.059 Deep Agents ACP Model Selection](../059-deepagents-acp-model-selection.md)
Depends on: card 165

## Goal

Close the route-local model-selection family with deterministic exact-package,
provider/access, argv, failure, lifecycle, and documentation proof.

## Work

1. Add or extend frozen fixtures for every delivered exact provider/model and
   operation row, including explicit selection and omission.
2. Prove prepared selection, access profile, plan, request, and child argv
   agree exactly before spawn.
3. Prove malformed/unknown selection, provider mismatch, missing/wrong key,
   provider rejection, duplicate argv, confirmation drift where applicable,
   cancellation, deadline, incomplete turn, and cleanup paths without leakage
   or fallback.
4. Prove one selected child/session lifetime and any Research 206-admitted fresh
   replacement behavior. Do not add load/resume or durable persistence.
5. Prove omission retains the existing empty argv and decoder corpus.
6. Update the Deep Agents guide, feature-matrix notes/cells only when
   warranted, package API baseline, Research 206, milestone/cards, and the
   reserved route-local closeout.
7. Run the named package, route, docs, API, example, doctor, and diff gates.

## Acceptance Criteria

- [ ] every delivered row has deterministic exact-package evidence
- [ ] selected and omitted argv are exact and non-duplicated
- [ ] provider/model/access agreement fails before spawn when mismatched
- [ ] invalid, unsupported, auth, provider, and confirmation failures never
      substitute an ambient/default value
- [ ] effective/observed model truth stays within Research 206's exact claim
- [ ] session/replacement truth is proved only where admitted
- [ ] resource, ambient isolation, host-owned credentials, permissions,
      callbacks, deadline, cancellation, terminal, and cleanup remain exact
- [ ] stable diagnostics disclose no key, prompt, output, raw provider payload,
      account identity, endpoint, or host path
- [ ] guide and matrix distinguish dispatch from effective/observed selection
- [ ] no other route, currentness claim, contract, release, or generation state
      changes
- [ ] `cargo fmt -p swallowtail-adapter-deepagents` passes
- [ ] `effigy validate:focused swallowtail-adapter-deepagents` passes
- [ ] `effigy package:verify-affected swallowtail-adapter-deepagents` passes
- [ ] `effigy check:examples`, `effigy qa:routes`, `effigy qa:northstar`,
      relevant index gates, `effigy package:api`, and `git diff --check` pass
- [ ] `effigy doctor` does not worsen the inherited baseline

## Stop Conditions

- any admitted row lacks deterministic provider/access/dispatch proof
- model/provider substitution or fallback remains possible
- validation reveals a contract/currentness dependency or breaking API

## Out Of Scope

- another feature family, provider prompt, release, merge, generation rollover,
  or g04 closure
