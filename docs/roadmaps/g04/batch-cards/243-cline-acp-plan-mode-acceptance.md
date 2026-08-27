# 243 Cline ACP Plan-Mode Acceptance

Status: complete
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.086 Cline ACP Plan Mode](../086-cline-acp-plan-mode.md)
Depends on: card 242

## Goal

Prove exact Cline ACP Plan negotiation, preserved omission, fail-closed drift,
same-session retention, fresh replacement, and unchanged authority/lifecycle
truth, then produce one review-ready route-local closeout.

## Scope

1. Add deterministic prepared-facade, protocol, driver, connection, and
   fixture coverage for the admitted row and every rejection boundary.
2. Assert exact package/behavior/mode membership, immutable plan/evidence/
   request agreement, `session/new` advertisement, one set-config request, and
   exact response confirmation before session return.
3. Prove missing `plan`, duplicate/foreign mode options, malformed snapshots,
   request rejection, missing/malformed/mismatched confirmation, disconnect,
   and protocol drift return no usable session and join all owned work.
4. Prove omission retains initialize/session-new/prompt bytes, emits no mode
   selection, and claims neither selected Plan nor provider-default Act.
5. Prove later turns on the same runtime remain on the manager created from
   Plan. No public post-start mutation or Plan-to-Act path may appear.
6. Prove fresh working-state replacement opens a new provider session,
   renegotiates the same immutable Plan selection, and remains context-losing
   `SessionReplaced`, not load, resume, or transcript restoration.
7. Prove Plan does not select `allow_always`, enable auto-approve, widen
   working-resource access, or imply tool, filesystem, network, shell,
   process, descendant, sandbox, model, account, or credential authority.
8. Preserve activity projection, permission observation, unexpected-write
   rejection, terminal, cancellation, malformed output, provider failure,
   disconnect, close, and joined cleanup behavior.
9. Update the Cline ACP guide, feature/route matrices, example when useful,
   package API baseline, changelog, Research 240, cards 242-243, g04.086, and
   the reserved route-local log. Record shared closeout deltas; do not edit the
   inventory, programme, triage, shared indexes, or sole Next Task.

## Acceptance Criteria

- [x] every admitted and rejected row has deterministic proof
- [x] selection cannot become ready or prompt before exact confirmation
- [x] omission, same-session turns, and fresh replacement remain exact
- [x] unsupported or drifted state never falls back to Act or another mode
- [x] load/resume and runtime mode mutation remain absent
- [x] provider behavior, permission, access, resources, configuration, and
      isolation stay separate
- [x] public docs claim only exact requested/dispatched/confirmed Plan truth
- [x] one review-ready worker PR contains the complete lane or honest stop

## Validation

```sh
cargo fmt -p swallowtail-adapter-cline
effigy validate:focused swallowtail-adapter-cline
effigy package:verify-affected swallowtail-adapter-cline
effigy check:examples
effigy qa:routes
effigy qa:northstar
effigy package:api
git diff --check
```

Focused package verification is required. Broad workspace tests, live probes,
MSRV, release, and consumer checks are not authorized by this card.

## Closeout Boundary

- Close cards 242-243 and g04.086 only when the exact row ships.
- Keep shared inventory/programme/index/Next Task updates for the orchestrator
  after merge.
- Keep g04 open. Do not select another family, roll the generation, merge the
  PR, or close g04.

## Stop Conditions

- exact fixtures contradict Research 240
- confirmation or lifecycle retention cannot remain exact
- cleanup no longer joins before resource release
- delivery needs live account/provider work, generic config, shared
  contract/runtime changes, currentness movement, or authority widening

## Out Of Scope

Another feature or route, live provider acceptance, currentness, release,
merge, generation rollover, or g04 closure.

## Closeout

Route-local acceptance and docs shipped with the worker PR. Shared inventory,
programme, triage, indexes, and Next Task remain for the orchestrator after
merge.
