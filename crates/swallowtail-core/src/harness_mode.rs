/// A portable harness execution posture selected by the consumer.
///
/// This does not grant filesystem, network, tool, or permission authority.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HarnessMode {
    Plan,
}
