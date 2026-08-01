# 2026-08-01 Codex Thread Catalogue And Import Driver

## Result

Card 053 is complete. Codex app-server now exposes resource-scoped thread
catalogue and explicit import roles behind its prepared facade for only the
qualified `0.105.0..=0.107.0` and `0.110.0..=0.146.0` segments.

## Production Shape

Listing resolves one host-approved working resource to an exact filesystem cwd,
then sends bounded `thread/list` requests with non-archived `cli`, `vscode`,
and `appServer` sources. Candidate ids are consumer-safe traversal identities;
provider ids, titles, previews, update times, activity, and availability remain
bounded records under the immutable catalogue plan.

Import opens a separate app-server connection and sends `thread/read` with
`includeTurns: true`. It validates bounded history and rechecks exact provider
id, cwd, source, update time, and inactive availability before issuing an
`ExplicitlyImported` binding. Missing, changed, active, substituted, and
wrong-resource observations fail without a binding.

The imported binding enters the existing Codex load and replay-free resume
paths. Import adds no prompt, consumer thread, persistence, background sync,
archive, or delete effect.

## Validation

- `effigy validate:focused swallowtail-adapter-codex` passed 154 tests
- deterministic coverage includes exact range advertisement, two-page cursor
  traversal, bounded redaction, resource mismatch, stale/missing/active/id
  drift, history validation, imported load, and imported resume
- no live provider, broad workspace, package, or consumer suite ran

## Next

Execute card 054. Run the common profile across local and remote-authoritative
hosts, close cancellation/deadline/process-loss/cleanup evidence, update public
prepared guidance, and compile the extracted Codex package.
