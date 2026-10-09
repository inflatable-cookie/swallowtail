# Copilot ACP No-Child Diagnostic Preparation

`preparation-record.json` binds one fake-only macOS no-child proof and the
data-only request proposed for a later `1.0.93` initialize diagnostic. The
record includes replayable relative scratch roles, the exact profile template,
the current OS and imported runtime-profile closure, and the fake, launcher,
and harness identities.

Regenerate the record only with the Effigy preparation selector on the same
macOS build and architecture:

```sh
effigy prepare:copilot-acp-no-child-diagnostic
```

Run the fake boundary proof with:

```sh
effigy validate:copilot-acp-no-child-diagnostic
```

The request stays data-only and `execution_authorized: false`. Fake success
does not establish original Copilot startup, host-login, Auto-model, auth, or
provider compatibility. A later original attempt requires separate exact
authority and independent review.
