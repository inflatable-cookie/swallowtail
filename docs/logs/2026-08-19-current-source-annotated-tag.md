# Current Source Annotated Tag

Date: 2026-08-19
Roadmap: `../roadmaps/g04/003-current-source-tag-before-readiness.md`
Card: `../roadmaps/g04/batch-cards/009-current-source-annotated-tag.md`

## Result

Annotated source tag `v0.3.3` exists at the CI-green candidate. Local and
remote peels resolve to `51d186208e75dca4c04f077dd7179ec3c2fafae9`. The
annotated tag object is `ca30b367e51a70c56b0998b27e7e660ba7145657`.
Immutable `v0.3.2` remains `702f355631bb6fe8fe6cb098f48887df8ef8ca43` →
`a859d56b47b1bc2975df7d0516ca96fd8e485b35`.

The tagged tree is the compatible 40-package, 47-route patch. OpenHands stays
a package without a production route. No Spec 011 facade types are in that
tree. No GitHub Release or crates.io publication exists.

This closeout commit on `main` is not the tag identity. The tag was not
moved.

## Tag CI

Tag-triggered run
https://github.com/inflatable-cookie/swallowtail/actions/runs/32309276223
passed all five jobs at head SHA `51d186208e75dca4c04f077dd7179ec3c2fafae9`.

The first attempt failed the Stable job on
`swallowtail-adapter-deepseek::driver::active_stream_cancellation_joins_before_session_credential_release`:
expected `Cancelled`, observed `ProviderFailed` with
`swallowtail.deepseek.stream_incomplete` / `TransportInterrupted`. The same
SHA had already passed PR CI
https://github.com/inflatable-cookie/swallowtail/actions/runs/32306909807
and the dispatched pre-tag run
https://github.com/inflatable-cookie/swallowtail/actions/runs/32308431817.
The failed jobs were rerun in place. The rerun passed. The tag was left on
the original commit.

## Authority

g04.003 is complete. Contract promotion may start. Facade implementation
stays planned until the new contract is active. Later implementation cards
must not treat this closeout SHA as the release identity.

## Next

Promote Spec 011 into Contract 057, then amend the named seams. Do not start
facade code in that promotion.
