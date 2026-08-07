impl ScriptedAppServerHandle {
    fn accept_input(&self, bytes: &[u8]) {
        let mut input = self.state.input.lock().expect("input lock is available");
        input.extend_from_slice(bytes);
        let mut lines = Vec::new();
        while let Some(newline) = input.iter().position(|byte| *byte == b'\n') {
            lines.push(input.drain(..=newline).collect::<Vec<_>>());
        }
        drop(input);
        for line in lines {
            let message: serde_json::Value =
                serde_json::from_slice(&line).expect("driver sends valid JSONL");
            self.state
                .messages
                .lock()
                .expect("messages lock is available")
                .push(message.clone());
            self.state.messages_changed.notify_all();
            self.respond(&message);
        }
    }
}
