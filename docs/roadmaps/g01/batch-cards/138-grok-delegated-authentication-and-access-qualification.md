# 138 Grok Delegated Authentication And Access Qualification

Status: backlog
Owner: Tom
Updated: 2026-07-24
Milestone: `../047-grok-build-maintained-acp-range.md`

## Objective

Decide whether exact Grok Build `0.2.111` can activate one pre-existing
subscription OAuth credential without launching sign-in, changing access
mechanism, exposing a secret, or claiming bounded read-only execution.

## Governing Refs

- Research 030 and 031
- Spec 003
- Contracts 010, 013, 015, 017, 023, 029, 032, and 033
- roadmap g01.047
- card 137

## Scope

1. Obtain explicit operator authorization for one separately gated
   no-prompt probe against a host-approved, already authenticated exact
   `0.2.111` Grok state.
2. Observe exact `initialize`, auth-method selection, `authenticate`, and
   `session/new` with no prompt or model request.
3. Prove whether the chosen provider method:
   - activates only the existing Grok subscription credential
   - avoids browser, device, terminal, external-helper, and API-key fallback
   - preserves endpoint audience and access mechanism
   - exposes no token or raw credential state
4. Record exact state mutation, failed-attachment behavior, and cleanup.
5. Promote Spec 003 into a narrow durable contract only if the evidence
   establishes an activation-only lifecycle.
6. Rebaseline the first route as explicit `AmbientHost` plus ambient harness
   configuration and durable local state. Publish no bounded read-only,
   sandbox, or containment claim.

## Boundaries

- no package installation, update, downgrade, or npm launcher execution
- no new login, reauthentication, device flow, logout, or account switching
- no prompt, model request, tool execution, or paid inference
- no API-key route or billing fallback
- no credential, account, host path, or raw provider payload in committed
  evidence
- no production driver

## Acceptance Criteria

- [x] operator authorization is explicit
- [ ] exact artifact and host-approved delegated state are bound
- [ ] activation and sign-in are machine-distinct
- [ ] missing, expired, interactive, or mechanism-changing auth fails closed
- [x] ambient authority is visible and no read-only guarantee is claimed
- [ ] card 139 is ready only after one exact release is qualified

## Validation

- exact artifact and state precondition check — stopped before launch
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `cargo fmt --all -- --check` — passed
- `effigy doctor` — unchanged inherited 19 oversized-file findings:
  12 warnings and 7 errors
- `git diff --check` — passed

Authentication-success, rejection, drift, and redaction fixtures remain
pending because no authorized credential state exists.

## Blocker

The operator authorized the narrow probe on 2026-07-24. The exact
`0.2.111` executable is available and matches the frozen SHA-256, but this host
has no installed `grok` command and no default Grok state directory. No
pre-existing subscription credential is available to activate.

The probe stopped before agent launch or `authenticate`. Creating a credential
would require login, which remains outside authorization. Maintained public
docs still do not match the exact artifact's advertised auth method ids, so
offline evidence cannot replace the missing state.

## Attempt Evidence

- exact executable SHA-256 matches card 137
- exact executable version: `0.2.111`
- default Grok state: absent
- installed Grok command: absent
- API-key environment fallback: absent and outside the selected route
- agent launches: zero
- authentication requests: zero
- session requests: zero
- provider or model requests: zero
- credential files read: zero

## Unblock Options

1. Operator independently installs exact `0.2.111` and signs in through the
   intended Grok subscription route, then resumes the already authorized
   no-prompt probe.
2. Wait for maintained xAI ACP authentication documentation matching the
   exact current artifact.
3. Select a different credential route through a separate operator decision.

## Stop Conditions

- activation can start or replace sign-in
- the exact method can switch to API-key or another endpoint audience
- successful `session/new` performs a model request
- credential or account material cannot be kept out of evidence
- the lifecycle needs authority beyond a narrow shared contract

## Auto-Continuation

No. Independently provisioned exact state or matching maintained evidence is
required.

## Hold Decision

The operator placed the Grok lane on hold on 2026-07-24 because no Grok account
is available for this proof. Resume only through an explicit later decision.
g01 closure moves the lane to the shared
[roadmap backlog](../../backlog/grok-build-maintained-acp-range.md). This card
stays with its source generation and retains the same evidence gate.
