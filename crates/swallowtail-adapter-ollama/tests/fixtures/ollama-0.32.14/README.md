# Ollama 0.32.14 currentness corpus

This secret-free identity corpus freezes host Ollama `0.32.9` and official
GitHub `v0.32.14` before Swallowtail widens the `ollama.runtime` native
attached claim.

Exact host Mach-O and GitHub tag identities live in `identity.json`. Selected
`api/types.go` chat, inventory, process, show, and options structs stay
byte-identical from `v0.32.1` through `v0.32.14`. The five selected native
routes remain registered. Whole-file `types.go` and `routes.go` hashes moved
on unselected content.

GitHub still marks plain `v0.32.2` and `v0.32.10` as prereleases. Those stay
named exclusions inside the raised window. The attached server was not
started. The installed host binary was not replaced. Official macOS app
archives were not downloaded.

No fixture contains a credential, host path, account identity, provider
payload, real model observation, or live inference response.
