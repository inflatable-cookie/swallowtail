# Kimi Code 0.29.1-0.29.2 currentness corpus

This secret-free delta corpus qualifies the selected Kimi Code surfaces added
after Swallowtail's `0.29.0` milestone.

Exact source:

- `0.29.1`: annotated tag `785c319619ad4cbf87d8598afaea36c989f6cb66`,
  commit `f4c3967a417a539372eadab6c809d27b8a14c005`
- `0.29.2`: annotated tag `57503c7c4d854f2c66ea32e10cba28b2c5715e9c`,
  commit `8a45f10eddbb35c317047e82e567cdb59a220b4f`
- repository: `https://github.com/MoonshotAI/kimi-code`

Selected source identity:

| Surface | `0.29.1` blob | `0.29.2` blob | Disposition |
| --- | --- | --- | --- |
| ACP adapter tree | `458380a0eb0a2248b79735c3ed48b3f632ad5de6` | same | unchanged from `0.29.0` |
| CLI options | `6e422c3e2756645271153d2575d95dbe0ac7dbcc` | same | stable `-p --output-format stream-json` command |
| default print runner | `abee29962229564c238601e2817a10201c06ff95` | same | default v1 headless path unchanged from `0.29.0` |
| JSONL renderer | `0e2f35238db066a13b53ad2cfff11bdff2f76724` | same | code behavior unchanged; comment-only delta from `0.29.0` |
| bearer middleware | `96add593c6ed5c146268d36cd381170917a33851` | same | authenticated local surface unchanged |
| REST metadata | `090712e27eff0c1616d015f20c52a69270d5d8e6` | same | exact server version remains observable |
| REST prompt schema | `88aebc496c439969a9b6c9015e962f57086c6e16` | same | profile, disabled tools, model, and reasoning unchanged |
| event schemas | `79de1337cab4346c399aa3dc098e0f8849a21678` | same | selected event payloads unchanged |
| WebSocket control | `4fef9de57a8467d7c492d1546ca3c7efeb58515b` | same | protocol stays v2; legacy subscribe remains accepted |
| model catalogue schema | `4ae71c2edbe95b8b1743e23df5119d58e8c24cf3` | `9327ef3505257b16e226006cf5654f8efcc67ae3` | list response retained; `0.29.2` adds unselected provider-write schemas |

`0.29.1` changes selected local-server behavior: configured model lists omit
the synthetic secondary-model entry, and session/workspace/config events are
broadcast to every established WebSocket connection. Swallowtail ignores
foreign-session global traffic and retains its subscribed session stream.
`0.29.2` keeps that selected behavior.

The JSONL fixtures are bounded projections of upstream renderer test cases.
They include assistant output, tool exchange, retry metadata, and the durable
resume hint. The selected headless route does not set
`KIMI_CODE_EXPERIMENTAL_FLAG`; experimental v2 print behavior remains outside
the guarantee.

`retained-execution.json` freezes the Contract 042 policy and failure matrix
for explicit Kimi managed recovery and maximum-one local-server active-turn
reattachment. It reuses the exact qualified retry event and WebSocket v2
cursor schemas. The corpus permits no prompt replay, replacement session,
credential reacquisition, or hidden second attachment.

No fixture contains a credential, endpoint, provider payload, real session id,
or account observation.
