# g04.009 Overlay Merged

Date: 2026-08-20
Roadmap: `../roadmaps/g04/009-model-presentation-overlay.md`
PR: https://github.com/inflatable-cookie/swallowtail/pull/8

## Result

PR 8 fast-forwarded onto `main` at `84f6aa5db503c05b200848880aed097d6da2251a`.
Review comments
https://github.com/inflatable-cookie/swallowtail/pull/8#issuecomment-5354924973
and
https://github.com/inflatable-cookie/swallowtail/pull/8#issuecomment-5355127919
are the canonical verdict. `v0.3.3` still peels to `51d18620`.

The connection-lifecycle facade through overlay is on `main`. First-proof
adapter wiring is not.

## Next

Compile g04.010 first-proof inventory. Do not dispatch an adapter worker
until that inventory names a bounded first tranche.
