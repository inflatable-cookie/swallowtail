/// Operation-scoped remote resource whose deletion is owned by a driver.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OwnedRemoteResourceKind {
    Environment,
    Session,
}
