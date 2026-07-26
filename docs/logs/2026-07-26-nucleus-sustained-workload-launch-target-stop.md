# Nucleus Sustained Workload Launch-Target Stop

Date: 2026-07-26
Card: `../roadmaps/g02/batch-cards/042-nucleus-sustained-workload-hardening.md`

## Outcome

Card 042 stopped after 10 provider turns. The exact Swallowtail, Nucleus,
Codex, access, model, fixture, and host tuple passed preflight. Effigy launched
the current Nucleus dev executable with the isolated proof environment.

The dev executable and an existing debug bundle both publish
`dev.nucleus.desktop`. Computer Use selected the bundle at
`target/debug/bundle/macos/Nucleus.app`, not the environment-bound dev process.
The selected bundle used normal Nucleus state.

## Effects

- 9 synthetic turns completed
- 1 synthetic turn was cancelled through the normal UI
- 18 synthetic message records carry the unique `sustained` marker
- isolated proof evidence remained at 14 prior pilot turns
- normal state received the 10 new terminal turns
- the fixture commit and read-only permissions remained unchanged
- no Nucleus or Nucleus-owned Codex child remained
- no provider payload, prompt text, credential, or raw identifier entered
  Swallowtail evidence

No record was deleted or logically rewritten during diagnosis. One initial
SQLite diagnostic used the CLI's default open mode and checkpointed normal
database metadata before all later queries switched to `-readonly`.

## Classification

This is not evidence of a Swallowtail transport or lifecycle defect. It is an
application-proof control failure: one UI automation identity did not identify
one environment-bound native process.

The 10 turns, 2 native launches and catalogue selections, and 2 provider
sessions count against consumption evidence but cannot satisfy the isolated
workload.

## Stop

Card 042 is paused. Do not make another provider call or delete normal-state
records until the operator decides:

1. whether to preserve the marked synthetic records or authorize a separately
   planned recovery
2. which exact native launch/control target proves the isolated environment
3. whether to reset the ceiling to 60 provider turns, 7 launches and catalogue
   selections, and 12 provider sessions while retaining 50 valid read-only
   outcomes and the 4-hour active-time limit

## Reset

The operator chose to preserve the marked synthetic records and approved the
proposed reset. Resume uses a rebuilt current debug bundle, an explicit
environment-bound executable launch, and full app-path UI control. One exact
PID must have only the isolated database open before the next turn.
