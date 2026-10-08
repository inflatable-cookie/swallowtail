# Antigravity headless static boundary evidence

`control-flow.json` freezes function addresses, direct-call edges, and literal
references recovered from the exact macOS ARM64 executables listed there. The
analysis script reads executable bytes only; it never launches them. It checks
their SHA-256 and size against the frozen `1.2.11` and `1.2.12`–`1.3.1`
identity fixtures before mapping functions.

Recreate the file from downloaded and separately digest-verified archives:

```sh
effigy validate:antigravity-headless-static-boundary /path/to/extracted-artifacts /path/to/control-flow.json
```

The extracted-artifact directory has one `VERSION/antigravity` file for each
version in `control-flow.json`. Do not treat synthetic self-test cases as
provider evidence or use these macOS mappings to infer another platform.
