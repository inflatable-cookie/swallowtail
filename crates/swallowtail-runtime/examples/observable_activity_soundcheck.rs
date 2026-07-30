use swallowtail_runtime::{
    ActivityId, ActivityKindClass, ActivityLifecyclePhase, ObservableActivityAvailability,
    PreparedOperationEvidence, RuntimeEvent, RuntimeEventKind,
};

/// Soundcheck may inspect route truth without committing to a rich work-log UI.
pub fn activity_availability(
    evidence: &PreparedOperationEvidence,
) -> ObservableActivityAvailability {
    evidence.observable_activity().availability()
}

/// Optional bounded progress derived from portable activity only.
pub struct StructuredProgress<'a> {
    pub activity_id: &'a ActivityId,
    pub kind: ActivityKindClass,
    pub phase: ActivityLifecyclePhase,
    pub label: Option<&'a str>,
}

pub fn optional_progress(event: &RuntimeEvent) -> Option<StructuredProgress<'_>> {
    let RuntimeEventKind::Activity(activity) = event.kind() else {
        return None;
    };
    Some(StructuredProgress {
        activity_id: activity.activity_id(),
        kind: activity.kind().class(),
        phase: activity.phase(),
        label: activity.label().map(|value| value.as_str()),
    })
}

/// Final output remains available when every activity event is ignored.
pub fn final_output(event: &RuntimeEvent) -> Option<&str> {
    if event.kind() == &RuntimeEventKind::OutputAvailable {
        event.content().map(|content| content.as_str())
    } else {
        None
    }
}

fn main() {}
