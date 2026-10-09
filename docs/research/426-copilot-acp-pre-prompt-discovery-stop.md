# Research 426: Copilot ACP Pre-Prompt Discovery Stop

Date: 2026-10-09

## Scope

Record the safe pre-prompt discovery gate for exact Darwin ARM64 Copilot CLI
ACP artifacts `1.0.80`, `1.0.81`, and `1.0.93`, using the operator-reported
existing account reference `betterthanclay`. The approved discovery scope
allows initialize, authentication when required, session creation, supported
model/configuration discovery, and joined close. It allows no prompt, model
generation, permission approval, tool action, qualification, or credential
extraction.

This is a finite stop before exact-artifact start. No original artifact was
launched, no keychain or host auth/configuration metadata was read, and no
authenticated request was made. Research 404's immutable package inventory
and execution record remain unchanged. The stop follows the exact-identity and
boundary gates in Contract 023 and the credential activation limits in
Contract 015.

## Frozen identity and reused preparation

The [Research 404 inventory](./404-copilot-cli-acp-offline-authentication-stop.md)
binds the wrapper, native package, and selected Darwin ARM64 executable for
each version. Research 425 rechecked those selected native hashes and records
the exact executable SHA-256 values:

| Version | Native executable SHA-256 |
| --- | --- |
| `1.0.80` | `fe779da7dd2342c1d23f0744873fa27d0251eaaee4dc6637fa53093639c0f3c9` |
| `1.0.81` | `0f2ba6429dbee9f5adcdc2ad09ded7f5a0511f5da9af10c3a0dbc6ed070f004f` |
| `1.0.93` | `df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1` |

The prior authenticated-proof fake and plan schema were reused. The harness
now has a pre-prompt mode that sends only `initialize`, `authenticate`, and
`session/new`, sanitizes model/configuration identifiers from the synthetic
session result, closes the child, and rejects prompt, permission/tool callback,
unapproved endpoint, wrong-account/service, over-budget, auth/config write,
and host-resource escape controls. Its macOS fake sandbox denies network,
Keychain access, fake host-home and repository reads, and writes outside task
scratch. These are fake-only controls; they do not establish behavior of any
frozen artifact.

## Official documentation checked

GitHub's current [Copilot CLI authentication guide](https://docs.github.com/en/copilot/how-tos/copilot-cli/set-up-copilot-cli/authenticate-copilot-cli)
names the macOS Keychain service `copilot-cli`, describes
`~/.copilot/config.json` as a plaintext fallback when the system keychain is
unavailable, and lists token environment variables, Keychain, then GitHub CLI
as credential sources. It also says the CLI may prompt to store credentials
in the fallback file. The [programmatic reference](https://docs.github.com/en/copilot/reference/copilot-cli-reference/cli-programmatic-reference)
documents `COPILOT_HOME`, `COPILOT_MODEL`, and persistent model settings.
These current documents do not establish the exact account selector, item
attributes, prompt-free fallback behavior, or credential path used by the
three frozen artifacts. We did not inspect any host item or config file.

The current [ACP server guide](https://docs.github.com/en/copilot/reference/copilot-cli-reference/acp-server)
says `session/new` carries parameters such as the working directory and MCP
servers, and that slash commands such as `/model` are sent as ordinary ACP
prompts. It also says an unavailable command is treated as ordinary prompt
text. It does not document a read-only ACP model catalogue request or a model
list in the `session/new` response for these frozen versions. That documentation
does not disprove an undocumented version-specific response field, but it
provides no safe prompt-free way to enumerate the account's model IDs. The
[Auto selection guide](https://docs.github.com/en/copilot/concepts/models/auto-model-selection)
says selection depends on supported models, plan, and policy, and that
availability can change. **Inference:** even if Auto is selected, it cannot
prove a single fixed underlying model for the separately approved permission
proof.

The reviewed official documentation does not provide a complete exact origin
allowlist for authentication and model discovery, nor a version-specific
guarantee that only the named Keychain item is queried. The current harness
denies all network and denies the global `com.apple.securityd` Mach service.
It has no exact-origin proxy and no Keychain item-level sandbox filter.
Allowing unfiltered egress or the whole host home would exceed the approved
boundary.

## Discovery result

The following prerequisites remain unestablished for `1.0.93`; the older
controls were not started:

| Gate | Result | Required evidence |
| --- | --- | --- |
| Existing login item | Generic current docs name service `copilot-cli`; exact frozen-artifact service/account query is unknown. | Version-specific proof of the vendor-owned read for the existing `betterthanclay` item, without exposing the value or prompting. |
| Authentication destinations | No complete exact origin set for the frozen authentication flow. | Exact per-version destinations for the allowed authenticate/session-new exchange, with an enforcement plan that fails closed on every other origin. |
| Model/configuration surface | Current docs provide no prompt-free ACP catalogue; session/new response for the frozen artifact is unknown. | Exact non-prompt session/new model/configuration fields, or another vendor-supported read-only discovery operation. |
| Account model access | No authenticated request was made. | Exact ACP model IDs returned for this account and an observable per-session binding; Auto alone is insufficient. |
| Containment | Fake deny-all checks pass; a narrow Keychain and endpoint policy has not been proved. | Fake-tested item and endpoint filters, scratch-only writes, host-home/repository denial, and bounded joined close before original start. |

The record therefore makes no statement about current entitlement, exact
model access, authentication success, or provider retry/fallback behavior.
No original protocol, endpoint, or cleanup observation exists. No prompt
allowance was consumed.

## Smallest next adaptation

Obtain an exact per-version, vendor-owned map for `1.0.93` that names the
existing login item query, every authentication/model-discovery origin, and a
prompt-free model/configuration response. Use it to prepare a fake-tested
containment plan that enforces those exact reads and origins while denying
host-home/repository access and all writes outside scratch. Then persist and
review one secret-free 60-second plan before any `1.0.93` original start. Do
not start `1.0.80` or `1.0.81` until the `1.0.93` gates pass.

For any later, separately approved permission proof, preserve the existing
proposal: one original invocation per version, at most 60 seconds each, one
ACP prompt per invocation, zero harness retry/resend or Auto fallback, at most
one permission request and one tool attempt, zero effects, and zero reviewer
live attempts. Egress must remain restricted to the exact reviewed origins;
any unlisted origin, model-generation request beyond that later one-prompt
budget, or containment failure stops the attempt. This discovery authority
does not approve that prompt proof.

## Result

Fake pre-prompt controls are prepared and pass through the existing Effigy
selector. Exact authentication/model discovery remains stopped before any
original launch because the keychain-item, endpoint, and prompt-free catalogue
gates are not established. The exact `1.0.80` route claim and every other
claim remain unchanged.
