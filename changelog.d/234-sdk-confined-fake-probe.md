Added a private Darwin SDK probe launcher with kernel filesystem, loopback
egress and fork restrictions, finite storage/output/deadline bounds, and
supervisor-observed cleanup against owned hostile fakes. SDK/native and Node
qualification stay unchanged. Original execution remains blocked by platform,
settings, custody and authority gates.

Extended the fake stage with Linux Landlock/seccomp enforcement, a bounded
local streaming endpoint and explicit fake administrator-settings injection.
Existing Ubuntu/macOS Node 22.23.2 jobs retain actual platform proof identities;
canonical administrator injection and interpreter custody remain separate gates.
CI mutation fixtures canonicalize their owned temporary roots before checking
source substitution, including macOS's `/var` temporary-path alias.
