use serde_json::Value;

/// Resolves the ACP-effective host model that `session/new` will use.
///
/// This route does not send a model on `session/new` or `session/prompt`.
/// OpenCode applies the selected default agent's `model` over the root
/// `model`. A provider definition without that effective default, or without
/// existing host auth for its provider, is not usable.
#[must_use]
pub fn resolve_usable_acp_model(config: &Value, auth: &Value) -> Option<String> {
    let selected = selected_model(config)?;
    let (provider, model_id) = split_provider_model(&selected)?;
    if !model_is_defined(config, provider, model_id) {
        return None;
    }
    if !provider_auth_present(auth, provider) {
        return None;
    }
    Some(selected)
}

fn selected_model(config: &Value) -> Option<String> {
    let agent_name = config
        .get("default_agent")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("build");
    let agent_model = config
        .pointer(&format!("/agent/{}/model", json_pointer_escape(agent_name)))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);
    agent_model.or_else(|| {
        config
            .get("model")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
    })
}

fn split_provider_model(selected: &str) -> Option<(&str, &str)> {
    let (provider, model_id) = selected.split_once('/')?;
    if provider.is_empty() || model_id.is_empty() {
        return None;
    }
    Some((provider, model_id))
}

fn model_is_defined(config: &Value, provider: &str, model_id: &str) -> bool {
    let pointer = format!(
        "/provider/{}/models/{}",
        json_pointer_escape(provider),
        json_pointer_escape(model_id)
    );
    config.pointer(&pointer).is_some()
}

fn provider_auth_present(auth: &Value, provider: &str) -> bool {
    auth.get(provider)
        .and_then(Value::as_object)
        .is_some_and(|entry| !entry.is_empty())
}

fn json_pointer_escape(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}

#[cfg(test)]
mod tests {
    use super::resolve_usable_acp_model;
    use serde_json::json;

    #[test]
    fn definition_without_default_is_not_usable() {
        let config = json!({
            "provider": {
                "kimi-for-coding": {
                    "models": { "k3": {} }
                }
            }
        });
        let auth = json!({ "kimi-for-coding": { "type": "api" } });
        assert_eq!(resolve_usable_acp_model(&config, &auth), None);
    }

    #[test]
    fn default_must_match_definition_and_existing_auth() {
        let config = json!({
            "model": "kimi-for-coding/k3",
            "provider": {
                "kimi-for-coding": {
                    "models": { "k3": {} }
                },
                "other": {
                    "models": { "cheap": {} }
                }
            }
        });
        let auth = json!({ "kimi-for-coding": { "type": "api" } });
        assert_eq!(
            resolve_usable_acp_model(&config, &auth).as_deref(),
            Some("kimi-for-coding/k3")
        );
    }

    #[test]
    fn records_the_default_not_a_sibling_definition() {
        let config = json!({
            "model": "other/cheap",
            "provider": {
                "kimi-for-coding": {
                    "models": { "k3": {} }
                },
                "other": {
                    "models": { "cheap": {} }
                }
            }
        });
        let auth = json!({
            "kimi-for-coding": { "type": "api" },
            "other": { "type": "api" }
        });
        assert_eq!(
            resolve_usable_acp_model(&config, &auth).as_deref(),
            Some("other/cheap")
        );
    }

    #[test]
    fn default_without_auth_is_not_usable() {
        let config = json!({
            "model": "kimi-for-coding/k3",
            "provider": {
                "kimi-for-coding": {
                    "models": { "k3": {} }
                }
            }
        });
        assert_eq!(resolve_usable_acp_model(&config, &json!({})), None);
        assert_eq!(
            resolve_usable_acp_model(&config, &json!({ "kimi-for-coding": {} })),
            None
        );
    }

    #[test]
    fn default_without_definition_is_not_usable() {
        let config = json!({
            "model": "kimi-for-coding/k3",
            "provider": {
                "kimi-for-coding": {
                    "models": { "other": {} }
                }
            }
        });
        let auth = json!({ "kimi-for-coding": { "type": "api" } });
        assert_eq!(resolve_usable_acp_model(&config, &auth), None);
    }

    #[test]
    fn selected_default_agent_model_overrides_root_model() {
        let config = json!({
            "model": "kimi-for-coding/k3",
            "agent": {
                "build": { "model": "other/cheap" }
            },
            "provider": {
                "kimi-for-coding": {
                    "models": { "k3": {} }
                },
                "other": {
                    "models": { "cheap": {} }
                }
            }
        });
        let auth = json!({
            "kimi-for-coding": { "type": "api" },
            "other": { "type": "api" }
        });
        assert_eq!(
            resolve_usable_acp_model(&config, &auth).as_deref(),
            Some("other/cheap")
        );
    }

    #[test]
    fn named_default_agent_model_overrides_root_and_build() {
        let config = json!({
            "model": "kimi-for-coding/k3",
            "default_agent": "plan",
            "agent": {
                "build": { "model": "other/cheap" },
                "plan": { "model": "other/plan" }
            },
            "provider": {
                "kimi-for-coding": {
                    "models": { "k3": {} }
                },
                "other": {
                    "models": { "cheap": {}, "plan": {} }
                }
            }
        });
        let auth = json!({
            "kimi-for-coding": { "type": "api" },
            "other": { "type": "api" }
        });
        assert_eq!(
            resolve_usable_acp_model(&config, &auth).as_deref(),
            Some("other/plan")
        );
    }

    #[test]
    fn default_agent_without_model_falls_back_to_root() {
        let config = json!({
            "model": "kimi-for-coding/k3",
            "default_agent": "plan",
            "agent": {
                "plan": { "description": "fixture" }
            },
            "provider": {
                "kimi-for-coding": {
                    "models": { "k3": {} }
                }
            }
        });
        let auth = json!({ "kimi-for-coding": { "type": "api" } });
        assert_eq!(
            resolve_usable_acp_model(&config, &auth).as_deref(),
            Some("kimi-for-coding/k3")
        );
    }

    #[test]
    fn unusable_agent_override_does_not_fall_back_to_root() {
        let config = json!({
            "model": "kimi-for-coding/k3",
            "agent": {
                "build": { "model": "other/cheap" }
            },
            "provider": {
                "kimi-for-coding": {
                    "models": { "k3": {} }
                },
                "other": {
                    "models": { "cheap": {} }
                }
            }
        });
        let auth = json!({ "kimi-for-coding": { "type": "api" } });
        assert_eq!(resolve_usable_acp_model(&config, &auth), None);
    }
}
