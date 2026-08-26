use swallowtail_core::{ModelId, ResourceAccess};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Exact Cursor-local read mode dispatched by one headless run.
///
/// Every qualified Cursor build accepts `plan` and `ask` as the only
/// `--mode` values. Both are provider behavior. Neither implies process
/// isolation, filesystem containment, working-resource authority, permission,
/// tool, approval, or network authority, and neither is reported by the
/// qualified stream. A read mode is selectable only with
/// [`ResourceAccess::Read`]; read-write runs dispatch no mode at all.
pub enum CursorHeadlessReadMode {
    /// Canonical `--mode plan`, the exact default for [`ResourceAccess::Read`].
    Plan,
    /// Canonical `--mode ask`, qualified on the exact Cursor builds only.
    Ask,
}

impl CursorHeadlessReadMode {
    pub(crate) const fn token(self) -> &'static str {
        match self {
            Self::Plan => "plan",
            Self::Ask => "ask",
        }
    }
}

pub(crate) const fn resolve(
    access: ResourceAccess,
    selection: Option<CursorHeadlessReadMode>,
) -> Option<CursorHeadlessReadMode> {
    match access {
        ResourceAccess::Read => match selection {
            Some(mode) => Some(mode),
            None => Some(CursorHeadlessReadMode::Plan),
        },
        ResourceAccess::ReadWrite => None,
    }
}

pub(crate) fn arguments(model: &ModelId, read_mode: Option<CursorHeadlessReadMode>) -> Vec<String> {
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
    if let Some(mode) = read_mode {
        arguments.extend(["--mode".to_owned(), mode.token().to_owned()]);
    }
    arguments
}

#[cfg(test)]
mod tests {
    use super::{CursorHeadlessReadMode, arguments, resolve};
    use swallowtail_core::{ModelId, ResourceAccess};

    #[test]
    fn read_and_write_profiles_never_select_force_or_implicit_sandboxing() {
        let model = ModelId::new("fixture-model").expect("model");
        let read = arguments(&model, resolve(ResourceAccess::Read, None));
        let write = arguments(&model, resolve(ResourceAccess::ReadWrite, None));
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
    fn ask_selection_renders_exactly_one_canonical_mode_argument() {
        let model = ModelId::new("fixture-model").expect("model");
        let ask = arguments(
            &model,
            resolve(ResourceAccess::Read, Some(CursorHeadlessReadMode::Ask)),
        );
        assert!(ask.ends_with(&["--mode".to_owned(), "ask".to_owned()]));
        assert_eq!(ask.iter().filter(|value| *value == "--mode").count(), 1);
        assert!(!ask.iter().any(|value| value == "plan"));
        assert!(!ask.iter().any(|value| value == "--plan"));
        assert!(ask.iter().any(|value| value == "--trust"));
    }

    #[test]
    fn explicit_plan_selection_matches_the_read_default_argv() {
        let model = ModelId::new("fixture-model").expect("model");
        assert_eq!(
            arguments(
                &model,
                resolve(ResourceAccess::Read, Some(CursorHeadlessReadMode::Plan))
            ),
            arguments(&model, resolve(ResourceAccess::Read, None))
        );
    }

    #[test]
    fn read_write_access_discards_any_read_mode_selection() {
        for selection in [
            None,
            Some(CursorHeadlessReadMode::Plan),
            Some(CursorHeadlessReadMode::Ask),
        ] {
            assert_eq!(resolve(ResourceAccess::ReadWrite, selection), None);
        }
    }

    #[test]
    fn parameterized_model_id_renders_one_model_argument() {
        let model =
            ModelId::new("claude-opus-4-8[context=1m,effort=high,fast=false]").expect("model");
        for read_mode in [
            resolve(ResourceAccess::Read, None),
            resolve(ResourceAccess::Read, Some(CursorHeadlessReadMode::Ask)),
            resolve(ResourceAccess::ReadWrite, None),
        ] {
            let arguments = arguments(&model, read_mode);
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
}
