# 2026-07-20 Kimi ACP Currentness And Session Contract

## Changed

- revalidated Kimi Code against provider release `0.28.1`, tag commit
  `0032545b65f95c139ecba5a48ba1b911844e1ffe`
- pinned the tagged ACP adapter `0.3.4`, exact SDK `0.23.0`, ACP wire `1`, and
  stable schema artifact `v1.19.1` independently
- promoted Research 006 and Contract 017
- separated new, load, and resume; load owns bounded historical replay before
  readiness while resume forbids replay
- bound persistent provider sessions to configured instance, host, model route,
  working resource, access, and expanded session policy
- defined bounded UTF-8 replacement callbacks under exact `ReadWrite` host
  authority
- separated callback authority from Kimi tool approval and process filesystem
  containment
- selected pre-existing isolated delegated harness auth; terminal login and
  configured non-OAuth provider credentials remain excluded

## Current State

Card 055 is complete. Card 056 is ready for deterministic Kimi ACP fixtures.
The production driver is not ready: Kimi's callback layer delegates directory,
metadata, glob, process, and other paths locally, and Kimi load/resume does not
enforce the request `cwd`.

The roadmap now carries a separate card 057 for provider-neutral execution-host
filesystem containment. Kimi driver and conformance work moved to cards 058 and
059.

## Evidence

- [Kimi Code `0.28.1`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.28.1)
- [Kimi ACP reference](https://moonshotai.github.io/kimi-code/en/reference/kimi-acp)
- [ACP v1 session setup](https://agentclientprotocol.com/protocol/v1/session-setup)
- [ACP v1 filesystem](https://agentclientprotocol.com/protocol/v1/file-system)
- [ACP schema `v1.19.1`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.19.1)

No binary, account, credential store, provider configuration, or live endpoint
was used.

## Risks

- only a prior exact Swallowtail binding may load or resume a Kimi session
- replay is provider history evidence, not consumer transcript authority
- write callback support does not prove a Kimi tool was approved or contained
- Kimi OAuth and configured downstream-provider credentials need separate
  configured instances and access profiles
- native session close remains absent

## Next

Execute card 056. Freeze the exact bounded protocol corpus without a Kimi
binary or credential. Keep process containment and production code out of that
batch.
