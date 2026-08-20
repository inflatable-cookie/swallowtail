# g04.006 Addable Catalog, Admission, And Config Fields

Date: 2026-08-20
Roadmap: `../roadmaps/g04/006-addable-catalog-admission-and-config-fields.md`
Cards: `../roadmaps/g04/batch-cards/016-addable-route-catalog.md`,
`../roadmaps/g04/batch-cards/017-instance-admission.md`,
`../roadmaps/g04/batch-cards/018-config-field-descriptors.md`

## Result

Consumers can assemble addable routes and admit configured instances.

`swallowtail-runtime` owns `AddableRouteCatalog` and `admit_instance`.
Consumers insert adapter-local descriptors. There is no umbrella registry.
Unavailable names a missing install, runtime, or host service. Unsupported
stays distinct. Absence of a descriptor means the consumer did not link that
adapter. Discovery candidates are not catalog rows and cannot be admitted.

Admission writes `AdmittedInstanceRecord` through `ConnectionLifecycleStore`.
Two instances of one family remain distinct ids. Admission does not prepare,
select a model, or change 047 readiness.

Config fields stay opaque `ConfigFieldRef` values. Public records and JSON
adapters carry no paths, URLs, or env bodies. Testkit fixtures stand in for
adapter-local descriptors. No production adapter crate changed.
`public-api-0.3.3` is unchanged.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-d429e52f`
Worker branch: `t3code/addable-catalog-admission`

PR: https://github.com/inflatable-cookie/swallowtail/pull/5

## Next

Merge stays operator-authorised. g04.007 stays planned.
