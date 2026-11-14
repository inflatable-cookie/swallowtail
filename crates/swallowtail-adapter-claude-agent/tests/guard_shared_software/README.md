# ACP-private shared software integration

`reviewed/` contains byte-identical source snapshots from task234 at
f9025d2197333c53aed2266627dfa39d05bff7e3 (review5903353390).
`snapshot.json` maps each copy to its original path and digest; `source.sha256`
and `plan.json` retain the reviewed record and manifest. Ownership stays with234.
Only `process_identity`/`validate_entry`, the entry, bootstrap and child facade
are consumed. No kernel adapter, namespace or canonical setup is invoked.

The real Rust test owner spawns a private software supervisor, `/bin/sleep`
group owner and checked ACP Node as siblings. Node joins sleep's group; the
owned SDK spawns the reviewed Python entry, which execs the owned captured
Node child. The ACP-private adapter checks that actual graph. It does not use
234's driver-under-group-owner stand-in or claim its model proves this graph.

Run `effigy validate:acp-shared-software` on Darwin arm64 with installed
compatible Node. The runner derives the exact runtime/library identities and
explicit host principals, without installation. Tests use only repository-owned
ACP/SDK/native/endpoint/settings fakes and the actual prepared paths. Their
software waits are separate from independent kernel subtree settlement.
`originalCommand` remains null. Complete vendor closure, Python/cache custody,
combined administrator discovery, kernel enforcement and original authority
are separate gates. Changes to the shared schema/kernel/job belong to234.
