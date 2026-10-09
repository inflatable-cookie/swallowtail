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
provider compatibility.

`authority-record.json` is the separate exact grant for one isolated original
`1.0.93` initialize. Prove the original-profile path with fakes first:

```sh
effigy validate:copilot-acp-no-child-original-diagnostic
effigy prepare:copilot-acp-no-child-original-diagnostic
```

`--execute-original` is a consumed one-shot worker entrypoint.
`execution-record.json` freezes the original result: initialize not reached,
vendor startup unknown, `failure_class` `sandbox-denial`. Reviewers re-run the
fake original-profile proof and inspect the committed authority, attempt, and
execution records. They must not start the original. The result does not prove
auth, Auto, permissions, cancel/no-effect, or supported versions. Fail-closed
no-child policy may prevent vendor startup and is not a route narrowing. The
exact `1.0.80` claim is unchanged.
