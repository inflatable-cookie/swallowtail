# Knowledge index

Current truth for Swallowtail. Every topic has exactly one owning file. Link
to it; don't restate it.

| Topic | Owner |
| --- | --- |
| Vision and non-goals | [vision.md](vision.md) |
| System shape and dependency direction | [architecture/](architecture/README.md) |
| Durable rules and interfaces | [contracts/](contracts/README.md) |
| Working rules | [contracts/001-working-rules.md](contracts/001-working-rules.md) |
| Repository authority and consumer boundaries | [contracts/002-repository-authority.md](contracts/002-repository-authority.md), [architecture/repository-authority-map.md](architecture/repository-authority-map.md) |
| How we release | [contracts/release.md](contracts/release.md) (rules: [Contract 036](contracts/036-crate-release-and-compatibility-boundary.md)) |
| Version currentness procedure | [operations/version-currentness-checkpoint.md](operations/version-currentness-checkpoint.md) (rules: [Contract 029](contracts/029-interface-version-qualification-and-compatibility.md)) |
| Rust quality profile and deviations | [contracts/rust-quality-profile.json](contracts/rust-quality-profile.json), [contracts/rust-quality-deviations.json](contracts/rust-quality-deviations.json) |
| Writing style | [contracts/writing-style.md](contracts/writing-style.md) |
| Specs, provisional or promoted | [specs/](specs/README.md) |
| Retired concepts | [retired.toml](retired.toml) |
| Open questions | [questions.md](questions.md) |

Product documentation and evidence that live outside this folder:

| Material | Location |
| --- | --- |
| Consumer integration guides, route and feature matrices | [`docs/guides/`](../guides/README.md) |
| Release compatibility notes and consumer handoff records | [`docs/releases/`](../releases/README.md) |
| Retained research evidence, cited by contracts and the feature matrix | [`docs/research/`](../research/README.md) |
| Public API, route and dependency baselines | `release-baselines/` |
| Frozen `v0.1.x` registry-candidate machinery | `release-candidates/` |
