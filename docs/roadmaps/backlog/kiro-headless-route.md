# Kiro Headless Route

Status: deferred
Owner: Tom
Source: g03.094 card 286; Research 153

## Deferred Work

Qualify Kiro `--no-interactive` headless as a sibling of the accepted
`kiro.acp` route. Card 286 retargeted g03.094 to installer-manifest ACP
`kiro-cli acp` and left headless deferred.

Existing `kiro.acp` production support remains unchanged.

## Promotion Gate

Promote only when:

- exact installed headless identity exists independently of ACP
- the selected command does not flatten onto `kiro.acp` or a generic print
  route
- the active generation has capacity for a second Kiro driver

There is no implied revisit date.
