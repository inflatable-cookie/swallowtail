use swallowtail_core::{ModelId, ResourceAccess};

pub(crate) fn arguments(model: &ModelId, access: ResourceAccess) -> Vec<String> {
    let mut arguments = [
        "--print",
        "--output-format",
        "stream-json",
        "--model",
        model.as_str(),
        "--trust",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    if access == ResourceAccess::Read {
        arguments.extend(["--mode".to_owned(), "plan".to_owned()]);
    }
    arguments
}

#[cfg(test)]
mod tests {
    use super::arguments;
    use swallowtail_core::{ModelId, ResourceAccess};

    #[test]
    fn read_and_write_profiles_never_select_force_or_implicit_sandboxing() {
        let model = ModelId::new("fixture-model").expect("model");
        let read = arguments(&model, ResourceAccess::Read);
        let write = arguments(&model, ResourceAccess::ReadWrite);
        assert!(read.ends_with(&["--mode".to_owned(), "plan".to_owned()]));
        assert!(!write.iter().any(|value| value == "--mode"));
        for arguments in [&read, &write] {
            assert!(arguments.iter().any(|value| value == "--trust"));
            for rejected in ["--force", "--yolo", "--sandbox", "--stream-partial-output"] {
                assert!(!arguments.iter().any(|value| value == rejected));
            }
        }
    }

    #[test]
    fn parameterized_model_id_renders_one_model_argument() {
        let model =
            ModelId::new("claude-opus-4-8[context=1m,effort=high,fast=false]").expect("model");
        let arguments = arguments(&model, ResourceAccess::Read);
        assert!(
            arguments
                .windows(2)
                .any(|pair| pair == ["--model", model.as_str()])
        );
        assert_eq!(
            arguments.iter().filter(|value| *value == "--model").count(),
            1
        );
    }
}
