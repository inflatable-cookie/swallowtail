# 328 Qoder Headless 1.1.54 Identity

Status: promoted; identity evidence before claim edit.

Owner: Tom
Date: 2026-09-17
Card: g05.080
Authority: Contract 029; Research 151, 200, 256, 318; the Qoder prepared
guide; and the official npm `@qoder-ai/qodercli` registry.

## Answer

Official npm stable `latest` is `1.1.54`, published
`2026-09-16T15:56:43.751Z`; beta is the separate
`1.1.54-beta.1` channel. The previous ceiling was `1.1.52`; published
successors `1.1.53` and `1.1.54` are frozen in the new Qoder corpus.

The selected route remains `qodercli --print --output-format stream-json
--permission-mode dont_ask --max-turns 8 --no-session-persistence --cwd
<cwd> <prompt>`. The official `1.1.30` change that made the forwarded argv
value the AgentLoop ceiling is reconfirmed at the current endpoint. The
operator's ruling permits a declared route-owned bound, so the claim card
will bind exactly `1.1.54` on a new private behavior revision with a
deliberate eight-turn ceiling. The old `1.1.25` point is not retained.

## Method and identity

Npm stable and dist-tags were re-probed on 2026-09-17. The official `1.1.52`,
`1.1.53`, and `1.1.54` tarballs were retrieved with `npm pack
--ignore-scripts` into `/tmp`, verified against registry integrity and shasum
values, extracted, and inspected without executing downloaded code. The
complete current-hop tree and selected-source hashes are frozen in
[`qoder-headless-1.1.54`](../../crates/swallowtail-adapter-qoder/tests/fixtures/qoder-headless-1.1.54/).
`1.1.55` is not published (npm `404`) and remains the synthetic later-stable
gap. No host `qoder` or `qodercli` is present; Node `22.23.2` satisfies the
package's `>=20.0.0` engine.

The package tree is 28 files at `1.1.52` and `1.1.53`, then 30 at `1.1.54`.
`qodercli.js` and the provider-internal worker runtime change at both hops;
the dispatcher, chat protocol, and postinstall script remain byte-identical.
`1.1.54` adds only the unselected `vendor/sites/artifact.json` and
`vendor/sites/sites.zip` paths. The selected print dispatch, stream-json
decoder inputs, `dont_ask`, no-session-persistence, abort, cleanup, and
forwarded max-turns semantics remain unchanged across these two hops.

## Turn-bound decision

The selected route's existing `--max-turns 8` is now an explicit adapter-owned
bound. Eight is the upper edge of the route's established short one-prompt
bounded posture: it leaves enough room for useful tool-assisted work while
preventing a provider AgentLoop from running effectively unbounded. This is a
route constant, not a caller option and not an output-token limit.

The reachable terminal shape is the stream-json `result` envelope with
`subtype: "error_max_turns"`, `is_error: true`, and `num_turns` equal to the
configured bound. The decoder maps it to
`swallowtail.qoder.headless.max_turns` and a provider-failed terminal.

The change is a private milestone because the accepted route contract now
includes the previously unbound AgentLoop ceiling. It does not add a public
operation, credential authority, provider session, retention, permission,
working-resource, cancellation, or cleanup surface.

## Sources

- official npm [`@qoder-ai/qodercli`](https://registry.npmjs.org/@qoder-ai/qodercli)
- frozen [`qoder-headless-1.1.54` corpus](../../crates/swallowtail-adapter-qoder/tests/fixtures/qoder-headless-1.1.54/)
- [Research 318](./318-qoder-headless-1-1-52-identity-stop.md)
- [Research 151](./151-qoder-headless-1-1-25-identity.md)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)

This identity record edits no production claim.
