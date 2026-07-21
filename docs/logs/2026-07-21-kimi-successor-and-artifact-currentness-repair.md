# 2026-07-21 Kimi Successor And Artifact Currentness Repair

## Changed

- proved that Kimi Code `0.28.1` is the latest maintained TypeScript successor,
  not a legacy Python pin
- corrected annotated tag-object ids that had been mislabeled as source commits
  for Kimi and ACP schema `v1.19.1`
- retained the full selected ACP behavior corpus: separate new, load with
  replay, resume without replay, prompt, write, cancellation, and joined close
- pinned the official macOS arm64 archive and extracted Mach-O digests
- recorded the valid upstream Developer ID signature and its dynamic-code
  entitlements; it lacks App Sandbox inheritance and must be deployment-signed
- froze `KIMI_CODE_HOME`, `KIMI_CODE_NO_AUTO_UPDATE=1`, non-ambient executable
  resolution, explicit access-instance separation, feature exclusions, and the
  prelaunch plus initialization upgrade gate

## Current State

Card 065 and Research 010 are complete and promoted. Contracts 015 and 017 do
not change. No Kimi binary, account, login state, or provider request enters
default tests.

Card 057 is the sole next task. It must verify and re-sign the pinned input,
prove the single-executable Node runtime remains compatible with App Sandbox
inheritance, propagate one security-scoped project grant through Kimi and every
descendant, isolate provider state, and join cleanup. Cards 058-059 remain
blocked behind containment.

## Validation

- 22 focused ACP tests pass
- focused warnings-denied lint and all-target compile pass
- full repository QA passes with 259 tests; two installed/live probes remain
  gated and ignored
- formatting, docs, Northstar, and diff checks pass
- doctor retains the inherited 19 oversized-file findings: 12 warnings and 7
  errors
