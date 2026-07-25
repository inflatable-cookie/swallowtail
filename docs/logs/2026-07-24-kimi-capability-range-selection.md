# Kimi Capability Range Selection

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/128-post-forward-compatibility-provider-coverage-evidence.md`

## Outcome

Kimi Code ACP is selected for the next compatibility tranche.

The claim will qualify exact `0.28.1` and `0.29.0` segments. It will not infer
continuous support between unpublished semantic versions. Exact stable newer
releases may run as unverified through the latest qualified behavior.

## Why

- all major original transport and lifecycle shapes now have production proofs
- `0.29.0` changes ACP reasoning from legacy `off`/`on` to model-declared
  effort levels while retaining legacy aliases
- the existing public runtime already has typed reasoning selection
- the missing boundary is narrow: applying one portable option through a
  negotiated harness channel
- the route needs no container, sandbox, foreign-language bridge, new access
  policy, or topology

Grok Build repeats proven ACP and JSONL shapes. Remote ACP moved to Active but
still lacks maintained SDK support and protocol hardening. Claude, Cursor, and
the older Kimi Agent SDK add foreign-runtime bridge weight. Kimi's web backend
is a provider UI surface, not the preferred integration authority.

## Contract Posture

Contract 034 permits only typed portable option mappings. Provider option ids,
labels, values, and snapshots remain adapter-private. The first proof applies
reasoning only to new sessions. Model switching, agent mode, tool gating,
custom agents, load/resume mutation, and generic provider configuration remain
excluded.

Existing Kimi delegated access and `AmbientHost` posture remain unchanged.
Sandboxing stays optional and independently selected.

## Continuation

- card 129: negotiated option records and exact two-release corpus
- card 130: installed discovery, range dispatch, and reasoning setup
- card 131: cross-topology conformance and closeout

## Validation

- `effigy qa:docs` passed
- `git diff --check` passed
- `effigy doctor` remains at the inherited 19 oversized-file findings:
  12 warnings and seven errors

## Sources

- [Research 028](../research/028-post-forward-compatibility-provider-coverage-selection.md)
- [Kimi Code releases](https://github.com/MoonshotAI/kimi-code/releases)
- [ACP remote transport lifecycle](https://agentclientprotocol.com/rfds/updates)
