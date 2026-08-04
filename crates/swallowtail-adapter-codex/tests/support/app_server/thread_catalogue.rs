impl ScriptedAppServerHandle {
    fn respond_thread_list(&self, message: &serde_json::Value, id: u64) {
        if matches!(
            self.mode,
            AppServerMode::ThreadCatalogue(ThreadCatalogueMode::Hold)
        ) {
            return;
        }
        if matches!(
            self.mode,
            AppServerMode::ThreadCatalogue(ThreadCatalogueMode::Disconnect)
        ) {
            self.state.closed.store(true, Ordering::SeqCst);
            return;
        }
        let cursor = message["params"]["cursor"].as_str();
        let requested_cwd = message["params"]["cwd"]
            .as_str()
            .expect("thread/list carries the materialized cwd");
        let wrong_resource = matches!(
            self.mode,
            AppServerMode::ThreadCatalogue(ThreadCatalogueMode::WrongResource)
        );
        let cwd = if wrong_resource {
            "/private/another/workspace"
        } else {
            requested_cwd
        };
        let (data, next_cursor) = if cursor.is_none() {
            (
                serde_json::json!([
                    {
                        "id": "thread-provider-import",
                        "name": "Imported thread",
                        "preview": "Bounded provider preview",
                        "updatedAt": 1775000000_u64,
                        "status": {"type": "notLoaded"},
                        "cwd": cwd,
                        "source": "cli"
                    },
                    {
                        "id": "thread-provider-active",
                        "name": "Active thread",
                        "preview": null,
                        "updatedAt": 1775000001_u64,
                        "status": {"type": "active"},
                        "cwd": cwd,
                        "source": "vscode"
                    }
                ]),
                serde_json::Value::String("private-thread-page-2".to_owned()),
            )
        } else {
            (
                serde_json::json!([{
                    "id": "thread-provider-idle",
                    "name": null,
                    "preview": "Second page",
                    "updatedAt": 1775000002_u64,
                    "status": {"type": "idle"},
                    "cwd": cwd,
                    "source": "appServer"
                }]),
                serde_json::Value::Null,
            )
        };
        self.state.push(serde_json::json!({
            "id": id,
            "result": {"data": data, "nextCursor": next_cursor}
        }));
    }

    fn respond_thread_read(&self, message: &serde_json::Value, id: u64) {
        let mode = match self.mode {
            AppServerMode::ThreadCatalogue(mode) => mode,
            _ => return,
        };
        if matches!(mode, ThreadCatalogueMode::Hold) {
            return;
        }
        if matches!(mode, ThreadCatalogueMode::Disconnect) {
            self.state.closed.store(true, Ordering::SeqCst);
            return;
        }
        if matches!(mode, ThreadCatalogueMode::Missing) {
            self.state.push(serde_json::json!({
                "id": id,
                "error": {"code": -32602, "message": "thread unavailable"}
            }));
            return;
        }
        let requested_id = message["params"]["threadId"]
            .as_str()
            .expect("thread/read carries a thread id");
        let thread_id = if matches!(mode, ThreadCatalogueMode::Mismatched) {
            "thread-provider-unrelated"
        } else {
            requested_id
        };
        let cwd = if matches!(mode, ThreadCatalogueMode::WrongResource) {
            "/private/another/workspace"
        } else {
            "/private/recording/workspace"
        };
        let status = if matches!(mode, ThreadCatalogueMode::Active) {
            "active"
        } else {
            "notLoaded"
        };
        let updated_at = if matches!(mode, ThreadCatalogueMode::Changed) {
            1775000001_u64
        } else {
            1775000000_u64
        };
        self.state.push(serde_json::json!({
            "id": id,
            "result": {
                "thread": {
                    "id": thread_id,
                    "updatedAt": updated_at,
                    "status": {"type": status},
                    "cwd": cwd,
                    "source": "cli",
                    "turns": [
                        {"id":"turn-1","status":"inProgress","items":[
                            {"type":"userMessage","id":"item-1","clientId":null,"content":[
                                {"type":"text","text":"Earlier question."}
                            ]}
                        ]},
                        {"id":"turn-2","status":"completed","items":[
                            {"type":"agentMessage","id":"item-2","text":"Earlier answer."}
                        ]}
                    ]
                }
            }
        }));
    }
}
