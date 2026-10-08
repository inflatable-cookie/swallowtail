# Research 415: Gemini CLI 0.63.0 ACP Policy Adaptation

Status: selected ACP qualification through the observed official stable
Owner: Swallowtail
Date: 2026-10-08
Task: swallowtail#141
Authority: Contract 023, Gemini ACP Permission And Filesystem Restrictions;
Contract 029; Research 372; exact ACP fixtures and route regressions

## Outcome

Qualify only `gemini-cli.acp` through the official stable `0.63.0`, with
version-specific behavior milestones. Preserve the existing `0.51.0..=0.61.0`
segment, its `0.56.1` and `0.59.1` exclusions, the independent headless claim,
and the exact `0.61.0` HTTP MCP live-honouring point. Research 372 remains the
historical currentness stop; this record adds the accepted adaptation and
qualification evidence without rewriting that stop.

## Current official stable and identity

Re-probed on 2026-10-08. The official npm `latest` tag for
`@google/gemini-cli` resolves to `0.63.0`; the GitHub releases page marks
`v0.63.0` as the latest non-prerelease stable. The newer preview/nightly
channel is not the stable target. The `0.63.0` package and tagged source
identities reproduce the frozen Research 372 corpus; no downloaded artifact
was executed.

| Version | npm published | npm tarball SHA-256 | GitHub commit / tree | Source archive SHA-256 | `gemini.js` SHA-256 | Release bundle SHA-256 | darwin-arm64 ZIP SHA-256 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `0.61.0` | 2026-09-24T00:04:53.021Z | `bc4efa5c925c4430105b552820ed3164bbeffa9dc227990fc922f954733bcd7d` | `bb523741c7429a44d03e964bc124c7c92df59d5f` / `c4226f654bb0b35109c4f5ce34535c5addecda47` | `917e0ac08eb3ef2048910ecc84d4da672ad0c58d82bb2eaa1d6f9e18af80241d` | `0b6e283ae88682b0e27e8ef85a608ab74807a1513dc0c053e5aa80d5b80b29ab` | `2a01f6f000a7ac060ff0c8e945c650c94cd05ddae52ad19ee0ada05a9bdba3ea` | `829738c00cab5a73b3ed01ce874c7ba79064fbe5869ad821517cba24f778dce2` |
| `0.62.0` | 2026-09-29T21:25:44.188Z | `2276032b1c33d2b828b1cf197e52f48e74b0a395326763ff01a80d97d0fbc0c3` | `b460678f3db508407554afd604cc9d6635becb2a` / `65ae43a5a0bf3670294be027ffe40eb5b0c5f18c` | `18d3955d07457723e5f9b24ff2d7622081b855ea8591d00a977b89a2089626f0` | `ef1d1bd9ee5aaae37ebcfb601b56659e3b16c5b258b21c138109c541e487ade7` | `68c199ca0c352ee3107e33d121faea24486f4416534224994357b567576e431c` | `050d9e3a53e6fc9cc2c00cd7d6771d72077b8fa2c0d1ec925ce3c353ee94b55b` |
| `0.63.0` | 2026-10-06T20:58:43.642Z | `97a6edfc10645463b517f0518d46a8c72efbdc12558a9a948607f726284a0420` | `573846625af9e93b3b968e0e0b86bb093a4c9b16` / `a6a023123538ba75eb8412528b9942d0de8917e3` | `75903470f15719bc061df6c2792cc55274494f7c25efdb7a864c50f5913bf917` | `5aee9ecb65b0b821e36a12f6e9e837301636de1a849a5a8bcc9c2b2620bbd72e` | `c9d66e50a0fe098a32486360308e558c472809da276caf2aac2769d8f10571c2` | `61babb895286b69a6112d7eab3f61aa0e2370ad6c1693a08badae8f64d9dd287` |

Research 372 and the frozen fixture
[`gemini-cli-acp-0.63.0`](../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-acp-0.63.0/)
retain complete npm file hashes, complete source-tree hashes, exact changed
path sets, and source classifications for both published hops after `0.61.0`.
The regression recomputes those sets and validates the full published chain:
`0.61.0` → `0.62.0` → `0.63.0`. Each package inventory has 449 files; the
source trees have 3,004, 3,014, and 3,018 regular files. The npm hops changed
50 added, 50 removed, and 6 modified files, then 48 added, 48 removed, and 5
modified files. The source hops contain 88 and 91 changed paths. Every
changed source path feeding the selected ACP wire, lifecycle, permission,
filesystem, failure, configuration, tool output, or usage surfaces remains
classified in the frozen surface ledger and checked by the identity
regression. The ACP SDK pin is `@agentclientprotocol/sdk@0.16.1`; wire version
remains `1`.

## Selected source behavior

At `0.62.0`, `packages/cli/src/acp/acpSession.ts` sends a `tool_call` update
with status `pending` before `session/request_permission`; cancellation is
followed by a correlated `tool_call_update` with status `failed`. These are
existing ACP v1 records and map to the adapter's existing activity lifecycle.

At `0.63.0`, `packages/cli/src/acp/acpSessionManager.ts` reuses its initialized
chat for `session/new`; this does not add a Swallowtail session or load
operation. `packages/core/src/safety/built-in.ts` requires `ASK_USER` for
`.gemini` configuration writes. `packages/core/src/policy/policy-engine.ts`
requires `ASK_USER` for matching command-prefix rules with shell redirection
unless `allowRedirection` is explicit, and its non-interactive path converts
`ASK_USER` to `DENY`. `packages/core/src/tools/shell.ts` prevents the provider's
`YOLO` forced-decision path from overriding an explicit ask. The route retains
its reject-and-cancel path and adds no permission callback or bypass, so these
operations stop at the provider permission boundary.

`packages/core/src/tools/read-file.ts` resolves and revalidates the real path
before reading. `packages/core/src/utils/paths.ts` protects `.env.*` names
except `.env.example`, `.env.sample`, `.env.template`, and `.env.dist`. The
adapter continues to serve only bounded host callbacks under the approved
resource. Its deterministic regressions exercise ordinary reads and writes,
the named environment-file exceptions, and refusal of traversal, symlink, and
absolute outside-root paths through `LocalProcessHost`. These checks establish
the callback boundary; they do not imply descendant-process containment.

The provider stores tool output up to 64 KiB and truncates additional text.
Untrusted context is wrapped by the provider; the adapter projects it as
opaque provider display content. Regressions drive the actual ACP driver and
host callback services with deterministic ACP fixtures. A large, frame-bounded
tool display verifies bounded opaque projection, correlation, typed failures,
cancellation, terminal outcomes, and joined cleanup. These fixtures verify
adapter handling, not provider execution; the frozen source identities and
classified source changes establish the provider's stored-output truncation
and untrusted-context behavior.

## Claim and boundaries

Keep claim id `gemini-cli.acp.window-1`, `AllowUnverified`, baseline, old
segment, and exclusions. Its qualified segments are now:

| Version | Behavior revision | Support |
| --- | --- | --- |
| `0.51.0..=0.61.0` | `gemini-cli.acp.v0.51.0` | maintained |
| exact `0.62.0` | `gemini-cli.acp.v0.62.0-tool-updates` | maintained |
| exact `0.63.0` | `gemini-cli.acp.v0.63.0-restricted-files` | maintained |

Published `0.56.1` and `0.59.1` remain incompatible. Later stable points such
as `0.63.1` remain visible `UnverifiedNewer`. The independent headless claim
stays at `0.51.0..=0.61.0`; its `0.62.0` and `0.63.0` points remain
unverified. The source-identical HTTP MCP mapping gains no live credit:
honouring remains proven only on exact host `0.61.0` under the previously
accepted profile.

Under Contract 036, this change adds no public API or lifecycle operation. The
documented `.63.0` permission and read-boundary restrictions are consumer
visible; they belong to the separately assessed pre-1.0 minor candidate, not
the urgent SDK patch. This qualification does not waive any release gate or
authorize a release, tag, publication, provider operation, or host mutation.

## Sources

- Official [npm package](https://registry.npmjs.org/@google%2Fgemini-cli) and
  [npm `0.63.0` artifact](https://registry.npmjs.org/@google%2Fgemini-cli/0.63.0)
- Official [GitHub stable releases](https://github.com/google-gemini/gemini-cli/releases)
  and [`v0.63.0` release](https://github.com/google-gemini/gemini-cli/releases/tag/v0.63.0)
- [Research 372: original identity and stop ledger](./372-gemini-cli-0-63-0-acp-currentness-stop.md)
- [Gemini ACP selection](../../crates/swallowtail-adapter-gemini/src/selection.rs)
  and [provider-free route regressions](../../crates/swallowtail-adapter-gemini/tests/gemini_cli_0_63_0_acp_policy.rs)
