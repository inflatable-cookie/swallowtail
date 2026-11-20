use crate::failure::malformed;
use swallowtail_acp_client::{
    AcpActivityConfig, AcpActivityProjection as SharedProjection, MessageClassification,
    PlanProjection,
};
use swallowtail_protocol_acp::AcpSessionUpdate;
#[cfg(test)]
use swallowtail_protocol_acp::AcpToolCallContent;
use swallowtail_runtime::{ActivityObservation, RuntimeFailure, RuntimeTurnId, TerminalStatus};

#[cfg(test)]
use std::collections::BTreeMap;

const COPILOT_ACTIVITY_CONFIG: AcpActivityConfig = AcpActivityConfig {
    id_prefix: "copilot-cli-acp",
    namespace_prefix: "copilot-cli.acp",
    plan_projection: PlanProjection::Ignore,
    message_classification: MessageClassification::Standard,
    malformed,
};

pub(crate) struct AcpActivityProjection {
    inner: SharedProjection,
    #[cfg(test)]
    assessment_diffs: BTreeMap<String, Vec<AssessmentToolDiff>>,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct AssessmentToolDiff {
    path: String,
    old_text: Option<String>,
    new_text: String,
}

#[cfg(test)]
fn extract_assessment_diffs(content: &[AcpToolCallContent]) -> Vec<AssessmentToolDiff> {
    content
        .iter()
        .filter_map(|item| match item {
            AcpToolCallContent::Diff {
                path,
                old_text,
                new_text,
            } => Some(AssessmentToolDiff {
                path: path.as_str().to_owned(),
                old_text: old_text.as_ref().map(|text| text.as_str().to_owned()),
                new_text: new_text.as_str().to_owned(),
            }),
            _ => None,
        })
        .collect()
}

impl AcpActivityProjection {
    pub(crate) fn new(turn_id: RuntimeTurnId) -> Self {
        Self {
            inner: SharedProjection::new(turn_id, COPILOT_ACTIVITY_CONFIG),
            #[cfg(test)]
            assessment_diffs: BTreeMap::new(),
        }
    }

    pub(crate) fn project(
        &mut self,
        update: &AcpSessionUpdate,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        #[cfg(test)]
        match update {
            AcpSessionUpdate::ToolCall(call) => {
                let diffs = extract_assessment_diffs(&call.content);
                if !diffs.is_empty() {
                    self.assessment_diffs
                        .insert(call.tool_call_id.as_str().to_owned(), diffs);
                }
            }
            AcpSessionUpdate::ToolCallUpdate(update) => {
                if let Some(content) = &update.content_replacement {
                    self.assessment_diffs.insert(
                        update.tool_call_id.as_str().to_owned(),
                        extract_assessment_diffs(content),
                    );
                }
            }
            _ => {}
        }
        self.inner.project(update)
    }

    pub(crate) fn complete(
        &mut self,
        terminal: &TerminalStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.inner.complete(terminal)
    }

    #[cfg(test)]
    pub(crate) fn is_pending_execute_for_assessment(
        &self,
        tool_call_id: &str,
        expected_path: &str,
        expected_old_text: &str,
        expected_new_text: &str,
    ) -> bool {
        self.inner.is_pending_execute(tool_call_id)
            && self
                .assessment_diffs
                .get(tool_call_id)
                .is_some_and(|diffs| {
                    diffs.iter().any(|diff| {
                        diff.path == expected_path
                            && diff.old_text.as_deref() == Some(expected_old_text)
                            && diff.new_text == expected_new_text
                    })
                })
    }
}
