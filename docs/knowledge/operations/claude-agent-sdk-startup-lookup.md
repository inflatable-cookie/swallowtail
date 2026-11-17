# Selected SDK startup absence regression

Retained source/fake evidence for the shared SDK/ACP startup handoff, separate
from [actual original qualification](claude-agent-sdk-trusted-qualification.md).
The [source receipt](claude-agent-sdk-startup-source.json) preserves exact frozen
SDK contexts and first-party guard strings; its historical status fields are not
new execution or qualification authority. Accepted scopes and counterpart source
are recorded in the [retirement provenance](claude-agent-sdk-prototype-retirement.md).

SDK318/319/321/322 make two uncaught module-scope optional
`process.getBuiltinModule("async_hooks")` probes. The private adaptation selects
the SDK’s own missing-API fallback by returning `undefined` for exactly one
`async_hooks` argument. It exposes no ambient builtin. Every other name, arity
and type refuses without coercion, and the property remains non-writable and
non-configurable. The ACP integration remains independently bound and qualified.

The focused owned regression reads those frozen contexts as inert data and
executes only owned source-shaped fixtures. It proves fallback selection and
refusal before effects for unauthorized lookups, unchanged/revoked guards and
invalid helper results. It supplies no vendor/native or hostile-sandbox credit.
The abandoned SDK host bootstrap and its proposal suites are removed.

```sh
effigy validate:claude-agent-sdk-startup-lookup
effigy check:claude-agent-sdk-startup-lookup
effigy check:format:claude-agent-sdk-startup-lookup
```

The actual SDK/native and shipped-sidecar permission, continued-turn,
cancellation and close evidence is owned by the trusted-original receipt.
