const ADMITTED: [u64; 3] = [1, 1_024, 65_536];

fn maximum(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).expect("fixture maximum is non-zero")
}

fn mode(value: &str) -> ReasoningMode {
    ReasoningMode::new(value).expect("fixture reasoning mode is valid")
}

fn planned_maximum(plan: &PreflightPlan) -> Option<Vec<CapabilityConstraint>> {
    plan.requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::OutputTokenLimit)
        .map(|required| required.constraints().cloned().collect())
}

fn plan_with_maximum(fixture: &LiveFixture, value: u64) -> PreflightPlan {
    let mut requirements: Vec<CapabilityRequirement> = fixture
        .plan()
        .requirements()
        .capabilities()
        .cloned()
        .collect();
    requirements.push(CapabilityRequirement::new(
        Capability::OutputTokenLimit,
        [CapabilityConstraint::OutputTokenMaximum(value)],
    ));
    fixture.plan_with_capabilities(requirements)
}
