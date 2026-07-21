use swallowtail_core::{PreflightPlan, SessionAccessPolicy, SessionRef};
use swallowtail_runtime::{SessionResumeBinding, WorkingResourceRef};

pub fn session_resume_binding(plan: &PreflightPlan, provider_ref: &str) -> SessionResumeBinding {
    session_resume_binding_for(
        plan,
        provider_ref,
        WorkingResourceRef::new("workspace.main").expect("working resource is valid"),
    )
}

pub fn session_resume_binding_for(
    plan: &PreflightPlan,
    provider_ref: &str,
    working_resource: WorkingResourceRef,
) -> SessionResumeBinding {
    SessionResumeBinding::new(
        SessionRef::new(provider_ref).expect("provider session ref is valid"),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id()
            .expect("session plan has a model route")
            .clone(),
        plan.model_id().expect("session plan has a model").clone(),
        working_resource,
        SessionAccessPolicy::read_only(),
    )
}
