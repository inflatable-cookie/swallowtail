# Provider-Session Management Binding Persistence

Status: deferred
Source: g02 card 060

## Gap

`ProviderSessionManagementBinding` is opaque and cloneable but has no stable
persistence codec. Consumers can execute management while the binding remains
in process. They cannot safely restore that authority after restart from a raw
provider session id.

## Promotion Gate

Promote only after current matrix capability work reaches a checkpoint and a
consumer requires provider archive, restore, or delete without retaining a
live binding object.

Promotion needs:

- a contract for versioned safe export and validated import
- no credential, raw provider payload, path, prompt, or transcript exposure
- exact driver, transport, instance, host, target, access, interface, resource,
  origin, and capability binding
- drift rejection before provider effects
- migration and revocation posture
- deterministic round-trip, tamper, redaction, and compatibility fixtures

Opening, loading, or resuming a provider session solely to recreate management
authority is not an implicit fallback.
