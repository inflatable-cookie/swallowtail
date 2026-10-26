# Claude Code 2.1.310 headless and response-only identity

Shared npm artifact evidence for two claims. Headless and response-only stay separate.

`dist-inventory.json` freezes the npm wrapper, Darwin arm64 package, and Linux x64 package from the response-only ceiling `2.1.293` through official stable `2.1.310`. `2.1.293` and `2.1.294` tarball integrity still matches the Research 422 and 423 inventories, so those trees were not re-extracted. Every later tarball was downloaded, checked against current npm SHA-512 integrity, and hashed per file. No downloaded executable was run.

`2.1.297` and `2.1.298` are absent from npm and from GitHub tags. `2.1.311` is the first absent later point.

`marker-ledger.json` records a byte search of each new platform `claude` for the selected stream, hook, safe-mode, and tool markers. The historical stream-json help lines stay present.

`headless-identity.json` and `headless-protocol.json` are the headless claim. `response-only-identity.json` and `response-only-protocol.json` are the response-only claim. At observation, npm `latest` and GitHub latest non-prerelease agree on `2.1.310`. npm `stable` remains the delayed channel at `2.1.308`. npm `next` is also `2.1.310`.

Hook and stream notes on each hop are classified in the protocol files. None of them changes the selected stream-json fields, the headless v2 hook rules, or the response-only text-only contract. Research 445 records the qualification.
