# 2026-08-28 g04.090a Goose ACP Mode Evidence

Status: done
Generation: g04
Card: 256
Research: 253

## Scope

Exact `goose.acp` `1.46.0` mode membership, selection, application,
confirmation, authority, failure, lifecycle, and omission evidence only.

## Boundary

Evidence only. This lane updated card 256, Research 253, this log, and
Goose-local frozen evidence under
`crates/swallowtail-adapter-goose/tests/fixtures/g04-090a-acp-mode/`.
Shared planning and production code stayed unchanged.

## Outcome

Honest empty deliver-now set for exact `goose.acp` `1.46.0` ACP mode
selection.

Tagged `v1.46.0` (`98c11ce2ee7b9b302978aa64b1eab7d0895607c7`) closes
membership at `auto|approve|smart_approve|chat`. `session/new` seeds
`currentModeId` from host `GOOSE_MODE` or default `auto`. Selection is
`session/set_mode` or `session/set_config_option` `{ configId: "mode" }`;
both call `on_set_mode` and persist session `goose_mode` without writing host
`GOOSE_MODE`. Unknown set values fail closed. Malformed persisted mode
reloads as **`auto`** (fail open). `auto` and `smart_approve` automatically
approve tools; `approve` still honors ambient host user AlwaysAllow; `chat`
skips tools and is not `HarnessMode::Plan`. Confirmation needs a live
provider-backed session; no goose on PATH; no live ACP. Omission keeps argv
exactly `goose acp` and leaves mode host-owned. Card 256 records the failed
drift fail-closed gate explicitly.

## Validation

```text
effigy validate:focused swallowtail-adapter-goose  # pass; 28 tests
effigy qa:northstar                                 # pass
git diff --check                                    # pass
```

## Next

PR against current `main`. Orchestrator owns shared closeout after merge
authorization. No production binding from this branch.
