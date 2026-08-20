# g04 Hosted OAuth Reassessment

Date: 2026-08-20
Roadmap: `../roadmaps/g04/README.md`
Research: `../research/169-first-proof-route-surface-inventory.md`

## Result

The first-proof-plus-consumer-path goal is complete. Hosted interactive
OAuth remains a remaining gate.

Contract 057 still names that proof as Anthropic or Claude subscription,
without extracting secrets. Current production access does not supply it:

- Anthropic Messages and Anthropic Managed Agent are public API-key
  pay-as-you-go. They reject subscription profiles.
- Claude Agent / Claude Code inherit local subscription or keychain state.
  That is installed, not hosted URL-open OAuth.
- Codex ChatGPT is cached local login on the installed app-server path.
- Grok, Kimi, Cursor, and Antigravity are delegated or local subscription
  logins on installed executables.

No production adapter registers `UrlOpen` or `LoopbackCallback`, or calls
`start_sign_in` with `SignInMethod::InteractiveOauth`. Those ports exist on
host-local test doubles only. Hosted production routes in the matrix are
API keys.

Reclassifying Claude or Codex as the hosted proof would contradict Research
169. Inventing an OAuth client Swallowtail does not own, or extracting
tokens from a local login, stays out of bounds.

The library-max loop is realized. The missing piece is a named production
route that actually performs hosted URL-open plus loopback without secret
extraction.

## Next

Define the next g04 lane. Do not compile hosted URL-open OAuth without a
no-secret-extraction proof and a named route.
