# 372 Gemini CLI 0.63.0 ACP Currentness Stop

Status: currentness stop; no claim change
Owner: Tom
Date: 2026-10-07
Task: swallowtail#096
Authority: Contract 029; Contract 063; Research 358, 362 and 369; Gemini
CLI ACP selection, prepared integration guide, driver, decoder and frozen
fixtures; official npm and GitHub channels

## Question

Can `gemini-cli.acp` extend its qualified `0.61.0` ceiling through current
official stable `0.63.0`?

## Answer

Not yet. Official npm `latest` and the newest GitHub stable release agree on
`0.63.0`. The published hops are `0.62.0` and `0.63.0`. Source review found
an ACP lifecycle extension at `0.62.0`, then permission and file-boundary
changes at `0.63.0` that can stop bounded-write operations under the route's
fixed reject-and-cancel permission policy. This is a consumer-visible
narrowing that needs an operator ruling or a contract-compatible adaptation.

Keep `gemini-cli.acp.window-1`, behavior revision
`gemini-cli.acp.v0.51.0`, baseline `0.51.0`, ceiling `0.61.0`, and the
existing unpublished exclusions `0.56.1` and `0.59.1`. Both published hops
remain `UnverifiedNewer`. This task does not change the headless claim.

## Method

Re-probed official npm `@google/gemini-cli` and GitHub release/tag channels on
2026-10-07. npm `latest` and the newest non-prerelease GitHub release are
`0.63.0`; the exact stable hops after `0.61.0` are `0.62.0` and `0.63.0`.
The npm tarballs, tagged source archives, GitHub release bundles, and unsigned
darwin-arm64 assets were downloaded into an isolated scratch directory and
verified by their published integrity or release digest. No downloaded
artifact was executed.

The host reports `0.61.0`; its `bundle/gemini.js` digest matches the official
npm `0.61.0` bin entry. The host was read for its version and digest only.
There was no prompt, login, credential use, installation, host update,
catalogue request, or ACP session.

`crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-acp-0.63.0/` freezes
the npm file hash maps, tagged-source regular-file hash maps, complete hop
path sets, and relevant source classifications. It reproduces Research 358's
`0.61.0` tarball, source archive, and bin-entry identities. Each source tree
contains one `docs/CONTRIBUTING.md` symlink, which is recorded outside the
regular-file inventory.

## Exact identities

| Version | npm published | npm tarball SHA-256 | GitHub commit / tree | Source archive SHA-256 | `gemini.js` SHA-256 | Release bundle SHA-256 | darwin-arm64 ZIP SHA-256 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `0.61.0` | 2026-09-24T00:04:53.021Z | `bc4efa5c925c4430105b552820ed3164bbeffa9dc227990fc922f954733bcd7d` | `bb523741c7429a44d03e964bc124c7c92df59d5f` / `c4226f654bb0b35109c4f5ce34535c5addecda47` | `917e0ac08eb3ef2048910ecc84d4da672ad0c58d82bb2eaa1d6f9e18af80241d` | `0b6e283ae88682b0e27e8ef85a608ab74807a1513dc0c053e5aa80d5b80b29ab` | `2a01f6f000a7ac060ff0c8e945c650c94cd05ddae52ad19ee0ada05a9bdba3ea` | `829738c00cab5a73b3ed01ce874c7ba79064fbe5869ad821517cba24f778dce2` |
| `0.62.0` | 2026-09-29T21:25:44.188Z | `2276032b1c33d2b828b1cf197e52f48e74b0a395326763ff01a80d97d0fbc0c3` | `b460678f3db508407554afd604cc9d6635becb2a` / `65ae43a5a0bf3670294be027ffe40eb5b0c5f18c` | `18d3955d07457723e5f9b24ff2d7622081b855ea8591d00a977b89a2089626f0` | `ef1d1bd9ee5aaae37ebcfb601b56659e3b16c5b258b21c138109c541e487ade7` | `68c199ca0c352ee3107e33d121faea24486f4416534224994357b567576e431c` | `050d9e3a53e6fc9cc2c00cd7d6771d72077b8fa2c0d1ec925ce3c353ee94b55b` |
| `0.63.0` | 2026-10-06T20:58:43.642Z | `97a6edfc10645463b517f0518d46a8c72efbdc12558a9a948607f726284a0420` | `573846625af9e93b3b968e0e0b86bb093a4c9b16` / `a6a023123538ba75eb8412528b9942d0de8917e3` | `75903470f15719bc061df6c2792cc55274494f7c25efdb7a864c50f5913bf917` | `5aee9ecb65b0b821e36a12f6e9e837301636de1a849a5a8bcc9c2b2620bbd72e` | `c9d66e50a0fe098a32486360308e558c472809da276caf2aac2769d8f10571c2` | `61babb895286b69a6112d7eab3f61aa0e2370ad6c1693a08badae8f64d9dd287` |

The ACP SDK pin stays `@agentclientprotocol/sdk@0.16.1`; wire version stays
`1`. Each npm package inventory contains 449 regular files. In the `.61` to
`.62` hop, 50 bundle files were added, 50 removed, and 6 files changed. In
the `.62` to `.63` hop, 48 bundle files were added, 48 removed, and 5 files
changed. The changed runtime entry is `bundle/gemini.js`; the other changed
files are package metadata and discovery changelogs/auth documentation.

Each tagged source tree is inventoried completely: 3,004 regular files at
`.61`, 3,014 at `.62`, and 3,018 at `.63`. The `.61` to `.62` source hop has
88 changed paths (10 added, 78 modified); `.62` to `.63` has 91 (4 added, 87
modified). The exact path sets and SHA-256 maps are in the fixture.

## Selected ACP deltas

At `0.62.0`, `packages/cli/src/acp/acpSession.ts` sends an existing ACP v1
`tool_call` update with status `pending` before `session/request_permission`.
When permission is cancelled, it sends an existing `tool_call_update` with
status `failed`. The wire method and status vocabulary are already represented
by the Swallowtail ACP activity projection. Provider-generated tool display
titles and explanations also change, while their existing ACP fields remain
opaque strings. The provider shell tool changes background-execution timing;
the outer Swallowtail process owner is unchanged.

At `0.63.0`, `acpSessionManager.ts` reuses an initialized chat in
`session/new`. Its `session/load` initialization order also changes, but load
is not selected or claimed. The `mcpServers` conversion within the file is
unchanged. The selected HTTP MCP mapping remains structurally the same, but
Research 362 proves live honouring on exact host `0.61.0` only; that evidence
does not transfer to either newer point.

## Permission and filesystem stop

The selected bounded-write profile starts Gemini with
`--approval-mode auto_edit`. At `0.63.0`:

- `safety/built-in.ts` makes changes to `.gemini` configuration paths require
  `ASK_USER`.
- `policy/policy-engine.ts` makes matching command-prefix rules with shell
  redirection require `ASK_USER` unless `allowRedirection` is explicit; its
  non-interactive path converts `ASK_USER` to `DENY`.
- `tools/shell.ts` prevents the `YOLO` forced-decision path from overriding an
  explicit `ask_user` decision.
- `tools/read-file.ts` sanitizes and resolves the real path, then revalidates
  access before reading. `utils/paths.ts` also treats `.env.*` names as
  protected except `.env.example`, `.env.sample`, `.env.template`, and
  `.env.dist`.

The route observes, rejects, and cancels provider permission requests. It has
no consumer approval callback. Therefore an operation that now reaches
`ASK_USER` cannot be completed through this route. This changes the effective
bounded-write guarantee for affected `.gemini` writes and redirected shell
commands. The real-path and `.env.*` checks also change the selected file-read
boundary. These are not new Swallowtail protocol methods; they are upstream
permission and filesystem behavior that may narrow existing consumer work.

The required adaptation is an operator ruling on whether those exact
`.63.0` outcomes may narrow the existing profile, or a contract-compatible
producer adaptation that retains the public route guarantee without adding a
consumer permission callback. After that decision, qualify the exact
mode-specific `.gemini` write, redirection, and real-path outcomes with
deterministic fixtures. Keep the existing reject-and-cancel behavior until
the ruling is recorded. Do not raise the version ceiling before those results
are settled.

## Other changed surfaces

The source ledger classifies every changed file reviewed as feeding ACP
lifecycle, provider tool display/output, permission, file access, subprocess,
provider config, or provider-owned auth/prompt behavior. The `.63.0` tool
result path adds 64 KiB stored-output truncation, resource/search text gains
untrusted-context wrapping, and provider history/subagent handling changes.
These can change provider-generated tool content, but add no Swallowtail
capability or ACP operation. The one-shot auth override is on a separate
provider relaunch path, outside the selected API-key profile. ACP HTTP MCP
honouring remains exact `0.61.0`; no live operation was attempted.

## Sources

- [npm `@google/gemini-cli@0.63.0`](https://registry.npmjs.org/@google%2Fgemini-cli/0.63.0)
- [GitHub `v0.63.0` release](https://github.com/google-gemini/gemini-cli/releases/tag/v0.63.0)
- [GitHub `v0.62.0` release](https://github.com/google-gemini/gemini-cli/releases/tag/v0.62.0)
- [GitHub `v0.61.0` release](https://github.com/google-gemini/gemini-cli/releases/tag/v0.61.0)
- frozen corpus: `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-acp-0.63.0/`
