use std::collections::BTreeMap;
use swallowtail_core::{
    ActivityCorrelationKind, ActivityDisclosure, ActivityKindClass, ActivityKindProfile,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture, ObservableActivityAvailability,
    ObservableActivityProfile,
};
use swallowtail_runtime::{
    ActivityCorrelation, ActivityId, ActivityLifecyclePhase, ActivityObservation,
    OrderedEventBuffer, RuntimeEvent, RuntimeEventKind,
};

pub(super) fn validate(
    profile: &ObservableActivityProfile,
    events: &[RuntimeEvent],
) -> Result<(), &'static str> {
    let mut buffer =
        OrderedEventBuffer::new(events.len().max(1)).map_err(|_| "event buffer was invalid")?;
    for event in events {
        buffer
            .push(event.clone())
            .map_err(|_| "event ordering or activity lifecycle was invalid")?;
    }

    let activities = activities_by_id(events);
    match profile.availability() {
        ObservableActivityAvailability::Unavailable
        | ObservableActivityAvailability::NotApplicable
            if !activities.is_empty() =>
        {
            return Err("unavailable activity profile emitted activity");
        }
        _ => {}
    }

    for observations in activities.values() {
        let first = observations[0];
        let class = first.kind().class();
        if class == ActivityKindClass::Unknown
            && profile.unknown_event_posture() != ActivityUnknownEventPosture::PreserveNamespaced
        {
            return Err("unknown activity was not permitted by the route posture");
        }
        let kind_profile = profile
            .kind(class)
            .ok_or("activity kind exceeded the prepared route profile")?;
        validate_observations(kind_profile, observations, events)?;
    }
    Ok(())
}

fn activities_by_id(events: &[RuntimeEvent]) -> BTreeMap<ActivityId, Vec<&ActivityObservation>> {
    let mut activities = BTreeMap::<ActivityId, Vec<&ActivityObservation>>::new();
    for event in events {
        if let RuntimeEventKind::Activity(observation) = event.kind() {
            activities
                .entry(observation.activity_id().clone())
                .or_default()
                .push(observation);
        }
    }
    activities
}

fn validate_observations(
    profile: &ActivityKindProfile,
    observations: &[&ActivityObservation],
    events: &[RuntimeEvent],
) -> Result<(), &'static str> {
    let phases = observations
        .iter()
        .map(|observation| observation.phase())
        .collect::<Vec<_>>();
    match profile.lifecycle() {
        ActivityLifecycleFidelity::CompleteLifecycle
            if phases.first() != Some(&ActivityLifecyclePhase::Started)
                || phases.last() != Some(&ActivityLifecyclePhase::Completed) =>
        {
            return Err("complete lifecycle omitted its start or completion");
        }
        ActivityLifecycleFidelity::UpdateAndCompletion
            if phases.first() != Some(&ActivityLifecyclePhase::Updated)
                || phases.last() != Some(&ActivityLifecyclePhase::Completed)
                || phases.contains(&ActivityLifecyclePhase::Started) =>
        {
            return Err("update-and-completion lifecycle had the wrong phases");
        }
        ActivityLifecycleFidelity::CompletionOnly
            if phases.as_slice() != [ActivityLifecyclePhase::Completed] =>
        {
            return Err("completion-only lifecycle emitted another phase");
        }
        ActivityLifecycleFidelity::Unavailable => {
            return Err("unavailable activity kind emitted an observation");
        }
        _ => {}
    }

    for observation in observations {
        if !disclosure_permitted(profile.disclosure(), observation.disclosure()) {
            return Err("activity disclosure exceeded the prepared route profile");
        }
        if observation.label().is_some()
            && observation.disclosure() == ActivityDisclosure::IdentityAndLifecycleOnly
        {
            return Err("activity label exceeded identity-only disclosure");
        }
        if let Some(content) = observation.content()
            && !profile
                .content_streams()
                .any(|stream| stream == content.stream())
        {
            return Err("activity content stream exceeded the prepared route profile");
        }
        if observation.task_list().is_some() && !profile.task_list_snapshots() {
            return Err("task-list snapshot exceeded the prepared route profile");
        }
        if let Some(correlation) = observation.correlation() {
            let kind = correlation_kind(correlation);
            if !profile.correlations().any(|supported| supported == kind) {
                return Err("activity correlation exceeded the prepared route profile");
            }
            validate_exchange_correlation(correlation, events)?;
        }
    }
    Ok(())
}

fn correlation_kind(correlation: &ActivityCorrelation) -> ActivityCorrelationKind {
    match correlation {
        ActivityCorrelation::Callback(_) => ActivityCorrelationKind::Callback,
        ActivityCorrelation::DirectToolCall(_) => ActivityCorrelationKind::DirectToolCall,
        ActivityCorrelation::ProviderRequest(_) => ActivityCorrelationKind::ProviderRequest,
    }
}

fn validate_exchange_correlation(
    correlation: &ActivityCorrelation,
    events: &[RuntimeEvent],
) -> Result<(), &'static str> {
    let matching_exchange = match correlation {
        ActivityCorrelation::Callback(expected) => events.iter().any(|event| {
            matches!(
                event.kind(),
                RuntimeEventKind::CallbackRequested(actual) if actual == expected
            )
        }),
        ActivityCorrelation::DirectToolCall(expected) => events.iter().any(|event| {
            matches!(
                event.kind(),
                RuntimeEventKind::DirectToolCallAvailable(actual) if actual == expected
            )
        }),
        ActivityCorrelation::ProviderRequest(_) => true,
    };
    if matching_exchange {
        Ok(())
    } else {
        Err("activity correlation did not match its separate exchange event")
    }
}

fn disclosure_permitted(maximum: ActivityDisclosure, actual: ActivityDisclosure) -> bool {
    matches!(
        (maximum, actual),
        (
            ActivityDisclosure::ProviderDisplayContent,
            ActivityDisclosure::ProviderDisplayContent
                | ActivityDisclosure::IdentityAndLifecycleOnly
        ) | (
            ActivityDisclosure::AdapterNormalizedSummary,
            ActivityDisclosure::AdapterNormalizedSummary
                | ActivityDisclosure::IdentityAndLifecycleOnly
        ) | (
            ActivityDisclosure::IdentityAndLifecycleOnly,
            ActivityDisclosure::IdentityAndLifecycleOnly
        )
    )
}
