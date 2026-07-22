# Pi RPC Provider Coverage Selection

Date: 2026-07-22

## Changed

- audited eighteen production descriptors and eleven common profiles
- refreshed Grok Build, remote ACP, Cursor, Claude Agent SDK, Pi, DeepSeek,
  Z.AI, Ollama, vLLM, and SGLang authority
- selected maintained Pi `0.80.10` RPC as the next harness proof
- promoted Contract 028 for scheduling and extension-UI relay
- compiled roadmap 035 and cards 099-101 inside g01
- kept DeepSeek V4 as the next direct-contract research target

## Decision

Pi adds strict language-neutral RPC, explicit prompt/steering/follow-up
scheduling, native abort, and correlated extension UI. Its first route is
`AmbientHost` with read-intent tools. No container, sandbox, filesystem
containment, provider default, model default, retry, or login mutation is
required.

The downstream provider and model remain configured-instance inputs. Pi's
maintainer support does not become support authority for every model provider.

DeepSeek V4 should not enter as another stateless compatible adapter. Its tool
flow requires provider reasoning continuation across explicit inference
attempts. That boundary needs research and a contract before code.

## Risks

- remote ACP remains Active, with remote SDK and hardening work incomplete
- Claude's current legal and Help Center guidance conflict on third-party
  subscription-backed Agent SDK use; no such access route is selected
- Cursor SDK remains public beta and needs a foreign-language runtime bridge
- attached Ollama, vLLM, and SGLang work must not absorb model management or
  Monkey authority
- DeepSeek legacy aliases retire on 2026-07-24

## Validation

- `effigy qa:docs` passes
- `git diff --check` passes
- doctor remains at the inherited 19 findings: seven errors and twelve
  warnings, with no new finding
- no account, credential, paid inference, browser login, installed harness,
  package mutation, or serving deployment was used

## Next

Card 099 is ready. Realize the shared scheduling/UI-relay records and freeze
the Pi `0.80.10` corpus before production process work.

