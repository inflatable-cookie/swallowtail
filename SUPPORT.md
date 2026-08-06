# Support

Swallowtail supports its documented Rust API, portable contracts, qualified
provider-interface ranges, deterministic examples, and explicit lifecycle and
failure semantics.

## Ask For Help Or Report A Bug

Use [GitHub Issues](https://github.com/inflatable-cookie/swallowtail/issues) for
integration questions and reproducible defects. Search existing issues first.

Include:

- exact Swallowtail tag or commit
- package and route ID
- provider, harness, SDK, or service version
- target and Rust version
- selected prepared facade and operation shape
- safe diagnostic code and portable failure classification
- smallest credential-free reproduction when possible
- expected and observed lifecycle, terminal, and cleanup behavior

Do not post credentials, tokens, account identifiers, private prompts,
consumer data, raw provider payloads, or private endpoints. Use
[SECURITY.md](SECURITY.md) for vulnerabilities.

## Boundaries

Swallowtail does not provide provider accounts, subscriptions, billing support,
models, harness installation, hosted infrastructure, application routing,
fallback policy, prompt design, tool implementations, or consumer UI support.

For an unsupported or unverified-newer provider version, include exact public
artifact evidence. Visibility as unverified newer is not a support guarantee.
Live provider checks remain optional and separately authorized; maintainers may
ask for deterministic corpus or source evidence instead.

The [route matrix](docs/guides/provider-route-matrix.md),
[integration guide map](docs/guides/integration-guide-map.md), and
[portable failure guide](docs/guides/portable-failure-handling.md) are the
fastest starting points.
