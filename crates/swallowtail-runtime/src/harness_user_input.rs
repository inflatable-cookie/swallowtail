#![deny(missing_docs)]

use crate::{HarnessQuestionId, HarnessQuestionOptionId, InputLimitExceeded, OperationContent};
use std::collections::BTreeSet;

/// Cardinality of a portable harness choice question.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessUserInputChoiceMode {
    /// At most one offered option may be selected.
    Single,
    /// Any number of distinct offered options may be selected.
    Multiple,
}

/// Portable shape of one harness-originated question.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessUserInputQuestionKind {
    /// Selection from a bounded set of stable options.
    Choice {
        /// Whether one or several offered options may be selected.
        mode: HarnessUserInputChoiceMode,
        /// Whether free text may accompany or replace an offered selection.
        allow_other: bool,
    },
    /// Free-text input with optional secret-display posture.
    Text {
        /// Whether consumers should conceal the entered value in their UI.
        secret: bool,
    },
}

/// One stable, bounded option offered by a harness question.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessUserInputOption {
    id: HarnessQuestionOptionId,
    label: OperationContent,
    description: Option<OperationContent>,
}

impl HarnessUserInputOption {
    #[must_use]
    /// Creates an option from its stable id, label, and optional description.
    pub const fn new(
        id: HarnessQuestionOptionId,
        label: OperationContent,
        description: Option<OperationContent>,
    ) -> Self {
        Self {
            id,
            label,
            description,
        }
    }

    #[must_use]
    /// Returns the stable option identity.
    pub const fn id(&self) -> &HarnessQuestionOptionId {
        &self.id
    }

    #[must_use]
    /// Returns the provider-visible option label as redacted operation content.
    pub const fn label(&self) -> &OperationContent {
        &self.label
    }

    #[must_use]
    /// Returns the optional provider-visible description.
    pub const fn description(&self) -> Option<&OperationContent> {
        self.description.as_ref()
    }

    fn byte_len(&self) -> usize {
        self.id.as_str().len()
            + self.label.byte_len()
            + self
                .description
                .as_ref()
                .map_or(0, OperationContent::byte_len)
    }
}

/// One ordered, bounded question requested by an interactive harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessUserInputQuestion {
    id: HarnessQuestionId,
    header: OperationContent,
    prompt: OperationContent,
    kind: HarnessUserInputQuestionKind,
    options: Vec<HarnessUserInputOption>,
}

impl HarnessUserInputQuestion {
    /// Creates a question after validating option shape and unique option ids.
    pub fn new(
        id: HarnessQuestionId,
        header: OperationContent,
        prompt: OperationContent,
        kind: HarnessUserInputQuestionKind,
        options: impl IntoIterator<Item = HarnessUserInputOption>,
    ) -> Result<Self, HarnessUserInputInvalid> {
        let options: Vec<_> = options.into_iter().collect();
        let valid_count = match kind {
            HarnessUserInputQuestionKind::Choice { .. } => !options.is_empty(),
            HarnessUserInputQuestionKind::Text { .. } => options.is_empty(),
        };
        let unique = options
            .iter()
            .map(HarnessUserInputOption::id)
            .collect::<BTreeSet<_>>()
            .len()
            == options.len();
        if !valid_count || !unique {
            return Err(HarnessUserInputInvalid);
        }
        Ok(Self {
            id,
            header,
            prompt,
            kind,
            options,
        })
    }

    #[must_use]
    /// Returns the stable question identity.
    pub const fn id(&self) -> &HarnessQuestionId {
        &self.id
    }

    #[must_use]
    /// Returns the short provider-visible question header.
    pub const fn header(&self) -> &OperationContent {
        &self.header
    }

    #[must_use]
    /// Returns the provider-visible question prompt.
    pub const fn prompt(&self) -> &OperationContent {
        &self.prompt
    }

    #[must_use]
    /// Returns the portable question shape.
    pub const fn kind(&self) -> HarnessUserInputQuestionKind {
        self.kind
    }

    /// Iterates offered options in provider order.
    pub fn options(&self) -> impl ExactSizeIterator<Item = &HarnessUserInputOption> {
        self.options.iter()
    }

    fn byte_len(&self) -> usize {
        self.id.as_str().len()
            + self.header.byte_len()
            + self.prompt.byte_len()
            + self
                .options
                .iter()
                .map(HarnessUserInputOption::byte_len)
                .sum::<usize>()
    }
}

/// Bounded ordered set of harness questions carried by one callback request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessUserInputRequest {
    questions: Vec<HarnessUserInputQuestion>,
    auto_resolution_ms: Option<u64>,
}

impl HarnessUserInputRequest {
    /// Creates a request after enforcing question, option, identity, and byte bounds.
    pub fn new(
        questions: impl IntoIterator<Item = HarnessUserInputQuestion>,
        auto_resolution_ms: Option<u64>,
        maximum_questions: usize,
        maximum_options_per_question: usize,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        let questions: Vec<_> = questions.into_iter().collect();
        if questions.is_empty() || questions.len() > maximum_questions {
            return Err(InputLimitExceeded::new(
                "harness user-input questions",
                maximum_questions,
                questions.len(),
            ));
        }
        let mut ids = BTreeSet::new();
        for question in &questions {
            if question.options.len() > maximum_options_per_question {
                return Err(InputLimitExceeded::new(
                    "harness user-input options",
                    maximum_options_per_question,
                    question.options.len(),
                ));
            }
            if !ids.insert(question.id()) {
                return Err(InputLimitExceeded::new(
                    "harness user-input question identities",
                    questions.len(),
                    questions.len() + 1,
                ));
            }
        }
        let actual = questions
            .iter()
            .map(HarnessUserInputQuestion::byte_len)
            .sum::<usize>();
        if actual > maximum_bytes {
            return Err(InputLimitExceeded::new(
                "harness user-input request",
                maximum_bytes,
                actual,
            ));
        }
        Ok(Self {
            questions,
            auto_resolution_ms,
        })
    }

    /// Iterates questions in provider order.
    pub fn questions(&self) -> impl ExactSizeIterator<Item = &HarnessUserInputQuestion> {
        self.questions.iter()
    }

    #[must_use]
    /// Returns the provider-supplied auto-resolution interval, when any.
    pub const fn auto_resolution_ms(&self) -> Option<u64> {
        self.auto_resolution_ms
    }

    #[must_use]
    /// Reports whether a response exactly answers this request's question shapes.
    pub fn accepts(&self, response: &HarnessUserInputResponse) -> bool {
        if response.answers.len() != self.questions.len() {
            return false;
        }
        self.questions.iter().all(|question| {
            response
                .answers
                .iter()
                .find(|answer| answer.question_id() == question.id())
                .is_some_and(|answer| question_accepts(question, answer))
        })
    }
}

/// One selected, textual, or skipped answer to a stable harness question.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessUserInputAnswer {
    question_id: HarnessQuestionId,
    selected_options: Vec<HarnessQuestionOptionId>,
    text: Option<OperationContent>,
    skipped: bool,
}

impl HarnessUserInputAnswer {
    #[must_use]
    /// Creates a non-skipped answer with selected options and optional text.
    pub fn selected(
        question_id: HarnessQuestionId,
        selected_options: impl IntoIterator<Item = HarnessQuestionOptionId>,
        text: Option<OperationContent>,
    ) -> Self {
        Self {
            question_id,
            selected_options: selected_options.into_iter().collect(),
            text,
            skipped: false,
        }
    }

    #[must_use]
    /// Creates an explicit skipped answer for one question.
    pub const fn skipped(question_id: HarnessQuestionId) -> Self {
        Self {
            question_id,
            selected_options: Vec::new(),
            text: None,
            skipped: true,
        }
    }

    #[must_use]
    /// Returns the stable question identity being answered.
    pub const fn question_id(&self) -> &HarnessQuestionId {
        &self.question_id
    }

    /// Iterates selected option identities in consumer-supplied order.
    pub fn selected_options(&self) -> impl ExactSizeIterator<Item = &HarnessQuestionOptionId> {
        self.selected_options.iter()
    }

    #[must_use]
    /// Returns optional free-text or other input.
    pub const fn text(&self) -> Option<&OperationContent> {
        self.text.as_ref()
    }

    #[must_use]
    /// Reports whether the consumer explicitly skipped the question.
    pub const fn is_skipped(&self) -> bool {
        self.skipped
    }

    fn byte_len(&self) -> usize {
        self.question_id.as_str().len()
            + self
                .selected_options
                .iter()
                .map(|id| id.as_str().len())
                .sum::<usize>()
            + self.text.as_ref().map_or(0, OperationContent::byte_len)
    }
}

/// Bounded complete response to one harness user-input callback.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessUserInputResponse {
    answers: Vec<HarnessUserInputAnswer>,
}

impl HarnessUserInputResponse {
    /// Creates a response after enforcing answer, identity, and byte bounds.
    pub fn new(
        answers: impl IntoIterator<Item = HarnessUserInputAnswer>,
        maximum_answers: usize,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        let answers: Vec<_> = answers.into_iter().collect();
        if answers.is_empty() || answers.len() > maximum_answers {
            return Err(InputLimitExceeded::new(
                "harness user-input answers",
                maximum_answers,
                answers.len(),
            ));
        }
        let unique = answers
            .iter()
            .map(HarnessUserInputAnswer::question_id)
            .collect::<BTreeSet<_>>()
            .len()
            == answers.len();
        if !unique {
            return Err(InputLimitExceeded::new(
                "harness user-input answer identities",
                answers.len(),
                answers.len() + 1,
            ));
        }
        let actual = answers
            .iter()
            .map(HarnessUserInputAnswer::byte_len)
            .sum::<usize>();
        if actual > maximum_bytes {
            return Err(InputLimitExceeded::new(
                "harness user-input response",
                maximum_bytes,
                actual,
            ));
        }
        Ok(Self { answers })
    }

    /// Iterates answers in consumer-supplied order.
    pub fn answers(&self) -> impl ExactSizeIterator<Item = &HarnessUserInputAnswer> {
        self.answers.iter()
    }
}

/// Indicates that a question's option shape is internally invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HarnessUserInputInvalid;

fn question_accepts(question: &HarnessUserInputQuestion, answer: &HarnessUserInputAnswer) -> bool {
    if answer.skipped {
        return answer.selected_options.is_empty() && answer.text.is_none();
    }
    let unique = answer
        .selected_options
        .iter()
        .collect::<BTreeSet<_>>()
        .len()
        == answer.selected_options.len();
    let offered = answer.selected_options.iter().all(|selected| {
        question
            .options
            .iter()
            .any(|option| option.id() == selected)
    });
    if !unique || !offered {
        return false;
    }
    match question.kind {
        HarnessUserInputQuestionKind::Choice { mode, allow_other } => {
            let count_ok = match mode {
                HarnessUserInputChoiceMode::Single => answer.selected_options.len() <= 1,
                HarnessUserInputChoiceMode::Multiple => true,
            };
            count_ok
                && (!answer.selected_options.is_empty() || answer.text.is_some())
                && (allow_other || answer.text.is_none())
        }
        HarnessUserInputQuestionKind::Text { .. } => {
            answer.selected_options.is_empty() && answer.text.is_some()
        }
    }
}

include!("harness_user_input/tests.rs");
