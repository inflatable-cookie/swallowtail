/// Configuration authority visible to one harness operation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HarnessConfigurationPosture {
    /// Accept the harness configuration sources visible under the host invocation.
    Ambient,
    /// Use an exact provider-qualified invocation that suppresses configuration.
    ProviderSuppressed,
    /// Use only configuration supplied through a separately bound host lease.
    HostScoped,
}
