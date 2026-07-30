#[cfg(test)]
mod tests {
    use super::{
        HarnessUserInputAnswer, HarnessUserInputChoiceMode, HarnessUserInputOption,
        HarnessUserInputQuestion, HarnessUserInputQuestionKind, HarnessUserInputRequest,
        HarnessUserInputResponse,
    };
    use crate::{HarnessQuestionId, HarnessQuestionOptionId, OperationContent};

    #[test]
    fn request_validates_stable_option_identity_and_other_text() {
        let question = HarnessUserInputQuestion::new(
            HarnessQuestionId::new("scope").unwrap(),
            OperationContent::new("Scope").unwrap(),
            OperationContent::new("Choose a scope").unwrap(),
            HarnessUserInputQuestionKind::Choice {
                mode: HarnessUserInputChoiceMode::Multiple,
                allow_other: true,
            },
            [HarnessUserInputOption::new(
                HarnessQuestionOptionId::new("tests").unwrap(),
                OperationContent::new("Tests").unwrap(),
                None,
            )],
        )
        .unwrap();
        let request = HarnessUserInputRequest::new([question], None, 3, 4, 1024).unwrap();
        let response = HarnessUserInputResponse::new(
            [HarnessUserInputAnswer::selected(
                HarnessQuestionId::new("scope").unwrap(),
                [HarnessQuestionOptionId::new("tests").unwrap()],
                Some(OperationContent::new("docs").unwrap()),
            )],
            3,
            1024,
        )
        .unwrap();

        assert!(request.accepts(&response));
        assert!(!format!("{request:?}").contains("Choose a scope"));
        assert!(!format!("{response:?}").contains("docs"));
    }

    #[test]
    fn request_rejects_foreign_option_and_wrong_cardinality() {
        let question = HarnessUserInputQuestion::new(
            HarnessQuestionId::new("mode").unwrap(),
            OperationContent::new("Mode").unwrap(),
            OperationContent::new("Choose one").unwrap(),
            HarnessUserInputQuestionKind::Choice {
                mode: HarnessUserInputChoiceMode::Single,
                allow_other: false,
            },
            [
                HarnessUserInputOption::new(
                    HarnessQuestionOptionId::new("a").unwrap(),
                    OperationContent::new("A").unwrap(),
                    None,
                ),
                HarnessUserInputOption::new(
                    HarnessQuestionOptionId::new("b").unwrap(),
                    OperationContent::new("B").unwrap(),
                    None,
                ),
            ],
        )
        .unwrap();
        let request = HarnessUserInputRequest::new([question], None, 3, 4, 1024).unwrap();
        let response = HarnessUserInputResponse::new(
            [HarnessUserInputAnswer::selected(
                HarnessQuestionId::new("mode").unwrap(),
                [
                    HarnessQuestionOptionId::new("a").unwrap(),
                    HarnessQuestionOptionId::new("foreign").unwrap(),
                ],
                None,
            )],
            3,
            1024,
        )
        .unwrap();

        assert!(!request.accepts(&response));
    }
}
