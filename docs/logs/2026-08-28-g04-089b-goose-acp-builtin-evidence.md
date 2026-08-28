# 2026-08-28 g04.089b Goose ACP Builtin Evidence

Status: done
Card: 253
Research: 250

## Boundary

Evidence only. This lane updated card 253, Research 250, this log, and
Goose-local frozen evidence under
`crates/swallowtail-adapter-goose/tests/fixtures/g04-089b-acp-builtins/`.
Shared planning and production code stayed unchanged.

## Outcome

Honest empty deliver-now set for exact `goose.acp` `1.46.0` `--with-builtin`.

Tagged `v1.46.0` (`98c11ce2ee7b9b302978aa64b1eab7d0895607c7`) clap accepts
any `--with-builtin` name on `goose acp` with no default. Stdio ACP does not
inherit `goose serve`'s omitted-`developer` default. Platform and
`goose_mcp::BUILTIN_EXTENSIONS` tables are source-closed, but docs cite
`github` outside those registries. With Swallowtail's `mcpServers: []`, host
enabled extensions still merge. Unknown names soft-fail into custom
`_meta.extensionResults` after provider/model `session/new`; they do not
reject at spawn. Platform deps can silently decline. Omission keeps argv
exactly `goose acp`. No install, configure, or live ACP probe.

## Validation

```text
effigy validate:focused swallowtail-adapter-goose  # pass; 28 tests
effigy qa:northstar                                 # pass
git diff --check                                    # pass
```

## Next

PR against current `main`. Orchestrator owns shared closeout after merge
authorization. No production binding from this branch.
