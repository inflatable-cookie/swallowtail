# Nucleus Native Pilot First Defect

Date: 2026-07-25

## Outcome

Card 041 started under the approved ChatGPT-backed 15-turn and 60-minute
ceiling. The first normal native launch confirmed:

- Codex `0.145.0`;
- the exact `gpt-5.4-mini` catalogue route;
- low reasoning;
- unchanged ChatGPT interactive OAuth and subscription audience; and
- the isolated read-only fixture and state boundary.

The first ordinary Agent Chat action then failed before provider-session open
or turn persistence:

`swallowtail.codex.preparation.tool_schema_limit`

Safe persisted evidence reports zero turns. No provider thread opened, no
Nucleus-owned Codex child remained, and no fixture, workspace, task, SCM,
forge, provider-account, push, publication, tag, or release mutation occurred.

## Ownership

This was a Swallowtail defect. The prepared Codex facade introduced fixed
limits of 4 tools and 4 KiB per input schema. Neither limit came from Contract
012 or the qualified Codex protocol.

The current official Codex manual says generated app-server schemas are exact
for the installed executable version. Codex 0.145.0's generated experimental
schema gives `dynamicTools` an array shape without `maxItems` and accepts
`inputSchema` without a byte-size constraint.

Swallowtail already requires `SchemaDocument::Inline`, validates JSON and
dialect, and receives finite bytes through the declaration constructor. The
prepared plan should record those concrete bounds, not claim an unsupported
provider maximum.

## Repair

Commit `54fbbc2af4e1615bed67815037aa2bcd6cc91dcb`:

- derives `ToolMaximumCount` from the declared tool count;
- derives `ToolMaximumSchemaBytes` from the largest bounded declaration;
- retains unique names, inline JSON Schema, valid JSON, and dialect checks;
- retains representability failures before provider work; and
- adds a two-tool regression with an input schema above 8 KiB.

The regression proves preparation, exact plan constraints, experimental API
negotiation, wire translation, and joined close. All 90
`swallowtail-adapter-codex` tests pass.

## Pilot Deviation

The failed launch made no model turn but consumed one authenticated catalogue
attempt. The original envelope cannot now prove three clean launches within
three physical launches.

Recommended reset:

- retain launch one as pre-turn defect evidence;
- permit 4 physical launches and 4 catalogue attempts total;
- run the unchanged 12-turn workload across 3 clean launches;
- keep 15 turns maximum, 6 provider threads, 3 live children, serial
  execution, read-only effects, and the original 60-minute ceiling; and
- make no further provider call before explicit approval.

## Next

The operator approved the narrow launch and catalogue reset on 2026-07-26.
Card 041 resumes with the paused interval excluded from its 60-minute
execution window. The prior catalogue remains attempt one of four.
