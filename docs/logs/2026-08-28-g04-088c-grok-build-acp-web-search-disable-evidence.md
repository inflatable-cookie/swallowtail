# 2026-08-28 g04.088c Grok Build ACP Web-Search Disable Evidence

Status: done
Card: 250
Research: 247

## Boundary

Evidence only. This lane updated card 250, Research 247, this log, and
Grok-local frozen evidence under
`crates/swallowtail-adapter-grok/tests/fixtures/g04-088c-web-search-disable/`.
Shared planning and production code stayed unchanged.

## Outcome

Honest empty deliver-now set for maintained `1.0.4..=1.0.5`.

Exact npm/platform digests match Research 163/219. Root
`--disable-web-search` parses before `agent stdio` and rejects on the ACP
subcommand; repeats fail closed. Unauthenticated initialize is identical with
and without the flag. Client `web_search`/`web_fetch` plus backend hosted
search application remain unfrozen from exact package source. The flag is not
`GROK_SANDBOX` / host-network containment. Omission keeps
`--no-auto-update agent stdio` and the matrix `No` search claim. Same stop
shape as Research 219 for `--no-subagents`.

## Validation

```text
effigy validate:focused swallowtail-adapter-grok  # pass; 30 tests
effigy qa:northstar                                 # pass
git diff --check                                    # pass
```

## Next

PR against current `main`. Orchestrator owns shared closeout after merge
authorization. No production binding from this branch.
