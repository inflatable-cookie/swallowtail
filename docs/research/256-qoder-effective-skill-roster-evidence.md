# 256 Qoder Effective Skill Roster Evidence

Status: promoted; empty deliver-now
Owner: Tom
Date: 2026-08-28
Updated: 2026-08-28
Roadmap: g05.002
Card: 004

## Question

Can exact Qoder headless `1.1.25` expose a complete prompt-free effective
skill roster for the selected run, including operator-installed global and
project-local skills, through its init `skills` and `plugins` fields?

## Boundary

Freeze official and exact package evidence without login, credentials, model
prompt, paid work, install/update, ambient host mutation, or recursive host
scanning. Return a closed deliver-now table or honest empty set. Do not change
production claims or code in this research card.

Method: reconfirm npm `@qoder-ai/qodercli@1.1.25` tarball digests from Research
151; source-review `bundle/qodercli.js` only; fetch official docs and the
upstream `skills` README for install-path tracing. Did not install Qoder, run
`qodercli`, run `npx skills`, log in, send `--print`, or scan host
`~/.qoder` / project trees.

## Decision

No. Research 256 admits an empty deliver-now set. Cards 005–006 stay planned.

Exact init `skills` and `plugins` are real selected-run fields on the
stream-json `system`/`init` frame, but the selected `qoder.headless` wire is
prompt-bearing, uses blocking auth, and cannot be claimed as a Contract 058
prompt-free complete roster from frozen evidence alone.

## Truth Layers

| Layer | Established | Not established |
| --- | --- | --- |
| Parsed | init schema and builder emit `skills: string[]` and `plugins: {name,path,source?}[]` | Live non-empty values on the selected route |
| Configured | discovery reads built-in, plugin, project `.qoder/skills`, user `~/.qoder/skills`, optional `.agents/skills` | That any specific host/project skill is present without a scan |
| Returned | builder maps `getSkillManager().getSkills().map(name)` and `getEnabledPlugins().map({name,path})` | Completeness under `--bare`, disabled filters, or untrusted project gates |
| Visible | fixtures show empty collections on a prompt-bearing success path | Positive global/project membership on selected-run init |
| Complete | source intends merged non-disabled skills as the init list | Contract 058 complete selected-context proof without prompt/auth |
| Inferred | docs/`npx skills` write into the same path classes discovery reads | That install membership equals selected-run visibility |

## Frozen Sources

Fetched or reconfirmed 2026-08-28. Tarball digests match Research 151 /
fixture `identity.json`.

| Source | Use | Identity |
| --- | --- | --- |
| npm `@qoder-ai/qodercli@1.1.25` tarball | exact package | SHA-256 `627749221c609bfb5514f4486fb42f464597cf49472ed52c087c36a1d2fbb4ab` |
| `bundle/qodercli.js` | init builder, discovery, headless auth, `--bare` | SHA-256 `77f7387974d5df79c7127bb41c9c7be8aad82aa567512ca2d9f780b2e3f73d52` |
| `package.json` | package identity | SHA-256 `459d820e451a6bdfd34c9799a841f2bcb66eaae155316e497cb1b12d44b53310` |
| [CLI Skills](https://docs.qoder.com/cli/Skills) | storage paths, startup load, `/skills` verify | SHA-256 `c1c231afb82751b781bb4762813036cc92c3b30a037b64b37b4e864e0622dbbc` |
| [CLI plugins](https://docs.qoder.com/cli/plugins) | plugin install scopes; plugin skills | SHA-256 `a3c41609ab7772386ad59befbf3c98b594a6c0b8cd3e98ef39bfd0f517b6dd40` |
| [CLI reference](https://docs.qoder.com/cli/cli-reference) | `--print`, `--plugin-dir`, `skills`/`plugins` subcommands | SHA-256 `4964336c9028d23e5ee002bf76a3caa09357d4ee09eeed80a4ad1032d4c10a65` |
| [Run in scripts](https://docs.qoder.com/cli/run-in-scripts) | headless `--print` / stream-json | SHA-256 `2e1d4dffe8ec8cfe3ab2d2ea2dcb0fcdfd52d7c6040324188949254b291ec831` |
| [Extensions skills](https://docs.qoder.com/extensions/skills) | `npx skills add … -a qoder` lead | SHA-256 `4e55c3ae61a51a1f70d3a8dba431703aff48b4202bfbb61d609381a2a50b8764` |
| [CLI commands](https://docs.qoder.com/cli/commands) | `/skills` is TUI; headless only for prompt commands | SHA-256 `b5b554ef567a135b7d40597946f194db3841c4ea0d6a5fce75465d7eaf28be9a` |
| [vercel-labs/skills README](https://raw.githubusercontent.com/vercel-labs/skills/main/README.md) | agent `qoder` → `.qoder/skills/` and `~/.qoder/skills/` | SHA-256 `0009a0d2fe1ebe225aafede3e719a0acf8b4bd2249bc4c94f95fc935b0c0d5b0` |
| Existing fixtures `success.jsonl` / `protocol.json` | empty init collections; prompt-bearing selected argv | corpus `qoder-headless-1.1.25` |

Local evidence freeze:
`crates/swallowtail-adapter-qoder/tests/fixtures/qoder-headless-1.1.25/skill-roster-evidence.json`.

## Evidence Answers

### 1. Exact code that populates init `skills` and `plugins`

Stream-json init builder `Bgt`:

- `skills` ← `config.getSkillManager().getSkills().map(skill => skill.name)`
- `plugins` ← `config.getEnabledPlugins().map(plugin => ({name, path}))`
- also emits model, cwd, session_id, tools, agents, slash_commands

Zod init schema: `skills: string[]`; `plugins: {name, path, source?}[]`.

`getSkills()` is `getMergedSkills().filter(skill => !skill.disabled)`.

### 2. Complete for selected model/run, or display-only?

Source intent: init lists merged non-disabled skill **names** and enabled
plugin `{name,path}` for that config session. It is not a UI-only field.

It is not a Contract 058-complete roster proof:

- names only; no provenance/description on the wire
- `--bare` forces `skills:[]` and `plugins:[]` even when managers hold data
- `allowedAgentSources` / untrusted project / `disableBuiltinSkills` / disabled
  names filter membership
- fixtures only record empty arrays on the selected prompt-bearing path
- no live selected-run positive membership without out-of-boundary effects

### 3. Sources that feed the collections

`discoverSkills` load order (later same-name wins via map set):

1. distribution built-in under `bundle/builtin/**/SKILL.md` (package contains
   `agent-creator`, `hook-config`, `sdk`, `skill-creator`)
2. vendor/security built-in path when enabled
3. enabled plugin skill dirs (`name` prefixed `pluginName:skill`)
4. project: `.qoder/skills`, `.qoder/commands`, optional `.agents/skills`, plus
   `--add-dir` trees
5. user: `~/.qoder/skills`, user commands, optional `~/.agents/skills`

`npx skills add … -a qoder` (official extensions docs + vercel-labs skills
README) installs into `.qoder/skills/` (project) and/or `~/.qoder/skills/`
(user). Those are the same path classes discovery reads. This card did not run
that installer. File presence after install is not selected-run visibility.

`plugins` init rows are enabled plugins, not the skill roster. Plugin-bundled
skills appear in `skills` when discovery loads them.

### 4. Frame before prompt acceptance and model inference?

Selected route argv always includes one prompt operand (`protocol.json` /
Research 151). Headless stream-json emits init through `emitInitMessage` while
handling that inbound prompt path. Auth-error helper `Ude` can also emit `Bgt`
before an `authentication_failed` assistant frame.

Contract 058: a prompt-bearing init is not prompt-free merely because it
precedes model output. Timing relative to first model HTTP call is therefore
insufficient.

### 5. Auth, durable allocation, mutation?

Mode table: `headless` → `auth: "blocking"`. Research 151: host-owned PAT or
persisted `qoder login` under `~/.qoder`; Swallowtail does not authenticate.
Selected `--no-session-persistence` avoids restoreable session write.

Sibling `qoder skills list` builds a `skills-list-session`, initializes
discovery, and prints names/descriptions/locations without a model prompt. It
is not the selected `qoder.headless` stream-json run, and was not executed
here.

### 6. Bounds, malformed, ordering, duplicates, freshness, empty

| Concern | Exact finding |
| --- | --- |
| Shape | skills must be `string[]`; plugins `{name,path,source?}` |
| Empty | legal; fixtures freeze `[]`/`[]` |
| Bare | `--bare` empties skills/plugins/tools/commands/agents on init |
| Disabled | filtered out of `getSkills()`; `getAllSkills()` retains them |
| Duplicates | discovery map keyed by name; cross-source conflicts emit warnings |
| Ordering | not a stable public contract beyond builder map iteration |
| Freshness | discovery at session initialize; TUI `/skills reload` for live sessions |
| Malformed SKILL.md | load errors / skip; not a decoder fixture in this corpus |

### 7. Binding dimensions

Init carries `model`, `cwd`, `session_id`, `qodercli_version`,
`protocol_version`, `permissionMode`. Changing cwd, config dir, enabled
plugins, trusted-folder posture, setting-sources, or bare mode changes the
observation. No fixture proves model-conditioned skill membership.

## Deliver-Now Table

| Row | Disposition | Reason |
| --- | --- | --- |
| Selected-run prompt-free complete roster via init `skills`/`plugins` | withhold | prompt-bearing selected wire; blocking auth; no positive complete proof inside boundary |
| Bind Contract 058 on `qoder.headless` now | withhold | honest empty set |
| Treat package `builtin/` or `npx skills` paths as visibility | withhold | distribution/install membership ≠ harness-declared selected-run roster |
| Use `qoder skills list` as the selected route | withhold | different command surface; not stream-json init; not executed |

## Stop Gates Hit

- selected init is prompt-bearing
- headless auth is blocking
- positive global/project membership needs live selected-run evidence or an
  ambient scan this card forbids
- completeness under filters/`--bare` is not closed for Contract 058

## Non-Goals

- production API, Contract 058 edits, cards 005–006
- live `--print`, login, `npx skills`, host mutation
- claiming empty fixtures as complete-empty selected-context proof for an
  arbitrary host with installed skills

## Closeout

Evidence-only. Empty deliver-now. Cards 005–006 remain planned. No production
claim change.
