# 2026-09-01 Kimi Code ACP QualifiedOnly Cap

Card: g05.017 batch 043
Gate: [containment and mediation](../triage/2026-09-01-kimi-code-acp-0-39-containment-and-mediation-gate.md)

## What changed

Operator A2 recorded. `kimi-code.acp` newer-version posture moved from
`AllowUnverified` to `QualifiedOnly`. Claim identity moved from
`kimi.acp.executable-window-2` to `kimi.acp.executable-window-5` (Contract 029:
posture and exclusion change invalidates the prior identity). Frozen historical
`window-3` and `window-4` are not reused. No ceiling raise,
no new behavior revision, no exclusion added or dropped.

- Segments stay exact `0.28.1` (Deprecated legacy) plus `0.29.0..=0.38.0`
  (Maintained declared-effort).
- Exact `0.39.0` and `0.39.1` stay excluded as recorded evidence.
- `KIMI_CODE_LATEST_QUALIFIED_VERSION` stays `0.38.0`.
- Unpublished `0.38.1`, excluded `0.39.0`/`0.39.1`, unpublished `0.39.2`, and
  `0.40.0` all assess `Incompatible`.
- Public assessment stays one `Incompatible` variant. Exclusion membership
  versus posture is distinguished through `exclusions()` and
  `newer_version_posture()`.

`kimi-code.headless`, `kimi-code.local-server`, Kimi Platform Chat, and every
second family are unchanged. Local-server remains `AllowUnverified`.

One standing-lane reopen trigger: a shipped-artifact identity run may reopen
planning only if every invocation path fails closed again for a terminal-less
client, or upstream supplies a ProviderEnforced boundary satisfying Contracts
017/023. The trigger authorizes a fresh identity/claim decision, never
automatic admission and never restoration of AllowUnverified by itself.

## Current state

The ACP axis is capped. A newly published point above `0.38.0` fails closed
instead of falling through to unverified-newer. The named exclusions remain
evidence of why the cap exists; they are not a growing deny-list. The live ACP
claim id is `kimi.acp.executable-window-5`. Evidence minted under
`kimi.acp.executable-window-2` fails closed at `observe_instance_update` before
projection. Frozen `window-3` (`kimi-code-0.30.0-0.31.0/installed-range.json`)
and `window-4` (`kimi-code-0.31.1/release.json`) remain historical, not live.
`git grep kimi.acp.executable-window-5 origin/main` and
`git log -S kimi.acp.executable-window-5 origin/main` returned no hits.

Falsification: flipping ACP back to `AllowUnverified` admits `0.38.1` /
`0.39.2` / `0.40.0` as `UnverifiedNewer`; dropping exclusion `0.39.0` fails
the exact classification proof while QualifiedOnly still refuses the point;
changing local-server posture to `QualifiedOnly` fails the isolation proof;
mutating the claim id back to window-2 would accept stale window-2 evidence;
reusing frozen window-3 or window-4 fails the reservation oracle.
Each mutation restores the original claim.

Public API surface is unchanged. No contract amendment, runtime/host change,
containment implementation, provider contact, auth, install, host mutation,
live probe, or downloaded-binary execution.

g05.009, card 034, and coverage 249/518 stay queued and unchanged.

## Next move

Fresh all-route Contract 029 currentness checkpoint. Do not rank the Kimi
family until that checkpoint. Do not treat the reopen trigger as admission.
