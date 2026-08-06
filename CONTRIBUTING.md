# Contributing

Contributions are welcome when they preserve Swallowtail's explicit provider,
authority, lifecycle, and evidence boundaries.

For support, use [SUPPORT.md](SUPPORT.md). Report vulnerabilities privately
under [SECURITY.md](SECURITY.md).

## Before Changing Code

1. Read [AGENTS.md](AGENTS.md) and [docs/README.md](docs/README.md).
2. Choose the exact package and route in the
   [provider route matrix](docs/guides/provider-route-matrix.md).
3. Confirm the behavior is already governed by an active contract. Research
   or contract work comes before runtime behavior when the boundary is missing.
4. Preserve provider differences through capabilities and typed prepared
   paths. Do not add a generic router, silent fallback, credential discovery,
   consumer policy, or provider-prose parser.

Provider and harness support requires exact public artifacts, bounded fixtures,
and deterministic regression evidence. Live authentication or prompts do not
replace that evidence and must never run implicitly.

## Development

Use Effigy for repository tasks:

```sh
effigy tasks
effigy test --plan
effigy validate:focused <workspace-package>
effigy package:verify-affected <workspace-package>
```

Group related changes into one meaningful batch. Run broad QA only when the
accepting roadmap calls for it. Do not commit credentials, local auth state,
provider captures containing private data, build output, or generated release
artifacts.

Code should:

- keep crates and modules focused
- keep provider-neutral vocabulary free of provider and consumer dependencies
- preserve exact route, model, operation, callback, and cleanup correlation
- fail closed without exposing raw provider data
- add deterministic coverage for each changed boundary
- document every supported public item

Documentation changes must keep contracts, route and feature matrices, guides,
examples, release copy, and the sole roadmap next task consistent.

## Validation

Use the narrowest accepted selector while developing. Before handoff, run the
validation named by the active card. Common documentation checks are:

```sh
effigy qa:docs
effigy qa:guides
effigy check:examples
```

Release mutations, workflow edits, authenticated provider work, consumer-repo
edits, and destructive provider-session management require separate authority.

## License

Unless stated otherwise, contributions are licensed under the repository's
[MIT License](LICENSE).
