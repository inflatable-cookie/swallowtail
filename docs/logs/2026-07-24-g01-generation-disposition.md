# g01 Generation Disposition

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/146-g01-generation-disposition.md`

## Outcome

The checkpoint recommends closing g01 at 49 roadmaps. It does not create
roadmap 050 or g02.

g01 remains active and paused until the operator chooses the next-generation
programme. Before closure, roadmap 047 and cards 138-141 must be rehomed
together as unchanged held work.

## Inventory

- roadmaps: 49 total, 48 completed, one on hold
- batch cards: 146 total, 142 completed, four on hold
- contracts: 35 active
- research: 32 promoted records
- specs: Spec 002 promoted, Spec 001 archived, Spec 003 provisional

Roadmap 047 and Grok cards 138-141 are the only unfinished implementation
surfaces. Card 137's exact artifact and protocol corpus remains valid. No Grok
release is qualified. Card 138 still requires independently provisioned
subscription state or matching maintained documentation; cards 139-141 remain
held behind it.

## Decision

Roadmap 050 is rejected. The Grok lane cannot advance from repository evidence,
and duplicating its hold would not create a coherent executable batch. Choosing
another provider merely to reach 50 would invent product policy.

Closure is preferred because g01 is at the upper edge of its documented
30-50-roadmap range and has completed its foundation, consumer adoption,
cross-adapter, transport, topology, lifecycle, and compatibility objectives.

The next-generation programme is not settled by current authority. Release
discipline and API stabilization, continued integration breadth, and other
directions have materially different product consequences.

## Spec Disposition

Spec 001 was stale: it still described runtime and adapters as proposals after
their architecture, contracts, roadmaps, and implementations were realized.
It is archived.

Spec 003 remains provisional. It applies only to held Grok delegated
authentication and cannot govern another provider implicitly.

## Validation

- exact roadmap, card, contract, research, and spec status audit
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `effigy doctor` — unchanged inherited 19 findings: 12 warnings, seven errors
- `git diff --check` — passed
- no provider, transport, protocol, host, or consumer implementation changed
- no g02 or roadmap 050 surface created
- held Grok evidence remains recoverable and unchanged

## Operator Gate

Approve g01 closure and choose the g02 programme. The boundary transition will
then rehome the held Grok lane, preserve Spec 003, close g01, and compile the
selected next-generation runway.

## Resolution

The operator approved closure on 2026-07-24 and selected API stabilization,
release discipline, packaging, and consumer upgrade support for g02. Roadmap
047 moved to the shared backlog, cards 138-141 remain with g01 as backlog
evidence, g01 is complete, and g02.001 is active.
