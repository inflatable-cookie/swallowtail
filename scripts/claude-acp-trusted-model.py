"""Owned Anthropic-compatible loopback peer. Never forwards a request."""

import http.server
import json
from pathlib import Path
import threading


class ModelFixture:
    def __init__(self, root, scenario):
        self.root, self.scenario = Path(root), scenario
        self.stop = threading.Event()
        self.rows = []
        self.lock = threading.Lock()
        fixture = self

        class Handler(http.server.BaseHTTPRequestHandler):
            def log_message(self, *_):
                pass

            def send_json(self, value, status=200):
                raw = json.dumps(value).encode()
                self.send_response(status)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(raw)))
                self.end_headers()
                self.wfile.write(raw)

            def do_GET(self):
                self.send_json(
                    {
                        "data": [
                            {
                                "id": "claude-sonnet-4-6",
                                "type": "model",
                                "display_name": "Owned fixture",
                            }
                        ],
                        "has_more": False,
                    }
                )

            def do_POST(self):
                try:
                    self.respond()
                except (BrokenPipeError, ConnectionResetError):
                    pass  # Expected when the real native turn is cancelled.

            def respond(self):
                count = int(self.headers.get("Content-Length", "0"))
                if not 0 < count <= 4 * 1024 * 1024:
                    self.send_json({"error": "request bound"}, 413)
                    return
                body = json.loads(self.rfile.read(count))
                if self.path.startswith("/turn/"):
                    self.send_json(
                        {
                            "text": "The read was denied; continuing."
                            if "denied" in self.path
                            else "Read allowed; continuing."
                        }
                    )
                    return
                if "count_tokens" in self.path:
                    self.send_json({"input_tokens": 100})
                    return
                if not self.path.startswith("/v1/messages"):
                    self.send_json(
                        {
                            "type": "error",
                            "error": {
                                "type": "invalid_request_error",
                                "message": "unselected fixture endpoint",
                            },
                        },
                        400,
                    )
                    return
                messages = body.get("messages", [])
                results = [
                    part
                    for message in messages
                    if isinstance(message.get("content"), list)
                    for part in message["content"]
                    if part.get("type") == "tool_result"
                ]
                last_user = next(
                    (m for m in reversed(messages) if m.get("role") == "user"), {}
                )
                continuing = isinstance(last_user.get("content"), list) and any(
                    p.get("type") == "tool_result" for p in last_user["content"]
                )
                secret_seen = "owned-secret-value" in json.dumps(results)
                row = {
                    "request": len(fixture.rows) + 1,
                    "model": body.get("model"),
                    "tools": [tool.get("name") for tool in body.get("tools", [])],
                    "toolResults": len(results),
                    "continuing": continuing,
                    "secretSeen": secret_seen,
                    "toolError": any(p.get("is_error") for p in results),
                }
                with fixture.lock:
                    if len(fixture.rows) >= 32:
                        self.send_json({"error": "request count bound"}, 429)
                        return
                    fixture.rows.append(row)
                    (fixture.root / "model-observations.json").write_text(
                        json.dumps(fixture.rows)
                    )
                if fixture.scenario == "error":
                    self.send_json(
                        {
                            "type": "error",
                            "error": {
                                "type": "invalid_request_error",
                                "message": "owned fixture error",
                            },
                        },
                        400,
                    )
                    return
                if fixture.scenario in ("cancel", "deadline", "native-exit"):
                    # Keep the real request pending until interrupt/stop, never a
                    # preloaded ACP/native response. The parent owns fixture join.
                    fixture.stop.wait(40)
                    return
                if continuing:
                    denied = not secret_seen
                    content = [
                        {
                            "type": "text",
                            "text": "The read was denied; continuing."
                            if denied
                            else "Read allowed; continuing.",
                        }
                    ]
                    reason = "end_turn"
                else:
                    content = [
                        {
                            "type": "tool_use",
                            "id": "owned_read_" + str(row["request"]),
                            "name": "Read",
                            "input": {
                                "file_path": str(
                                    fixture.root / "outside" / "secret.txt"
                                )
                            },
                        }
                    ]
                    reason = "tool_use"
                message = {
                    "id": "msg_owned",
                    "type": "message",
                    "role": "assistant",
                    "model": body.get("model", "claude-sonnet-4-6"),
                    "content": content,
                    "stop_reason": reason,
                    "stop_sequence": None,
                    "usage": {"input_tokens": 100, "output_tokens": 10},
                }
                if not body.get("stream"):
                    self.send_json(message)
                    return
                self.send_response(200)
                self.send_header("Content-Type", "text/event-stream")
                self.send_header("Connection", "close")
                self.end_headers()

                def event(name, value):
                    self.wfile.write(
                        (
                            "event: " + name + "\ndata: " + json.dumps(value) + "\n\n"
                        ).encode()
                    )
                    self.wfile.flush()

                event(
                    "message_start",
                    {
                        "type": "message_start",
                        "message": {
                            **message,
                            "content": [],
                            "stop_reason": None,
                            "usage": {"input_tokens": 100, "output_tokens": 0},
                        },
                    },
                )
                for index, part in enumerate(content):
                    initial = (
                        {**part, "text": ""}
                        if part["type"] == "text"
                        else {**part, "input": {}}
                    )
                    event(
                        "content_block_start",
                        {
                            "type": "content_block_start",
                            "index": index,
                            "content_block": initial,
                        },
                    )
                    delta = (
                        {"type": "text_delta", "text": part["text"]}
                        if part["type"] == "text"
                        else {
                            "type": "input_json_delta",
                            "partial_json": json.dumps(part["input"]),
                        }
                    )
                    event(
                        "content_block_delta",
                        {"type": "content_block_delta", "index": index, "delta": delta},
                    )
                    event(
                        "content_block_stop",
                        {"type": "content_block_stop", "index": index},
                    )
                event(
                    "message_delta",
                    {
                        "type": "message_delta",
                        "delta": {"stop_reason": reason, "stop_sequence": None},
                        "usage": {"output_tokens": 10},
                    },
                )
                event("message_stop", {"type": "message_stop"})

        self.server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.server.daemon_threads = False
        self.thread = threading.Thread(target=self.server.serve_forever)
        self.thread.start()

    @property
    def url(self):
        return "http://127.0.0.1:" + str(self.server.server_port)

    def close(self):
        self.stop.set()
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(5)
        if self.thread.is_alive():
            raise RuntimeError("owned fixture thread did not join")


if __name__ == "__main__":
    import sys

    if sys.argv[1:] == ["--self-test"]:
        import tempfile
        import urllib.request
        import urllib.error

        with tempfile.TemporaryDirectory(prefix="acp239-model-") as directory:
            root = Path(directory)
            peer = ModelFixture(root, "allow")

            def request(body):
                return urllib.request.urlopen(
                    urllib.request.Request(
                        peer.url + "/v1/messages",
                        json.dumps(body).encode(),
                        {"Content-Type": "application/json"},
                    ),
                    timeout=3,
                ).read()

            try:
                raw = request(
                    {
                        "model": "owned-model",
                        "stream": True,
                        "messages": [{"role": "user", "content": "owned read"}],
                    }
                )
                events = [
                    json.loads(line[6:])
                    for line in raw.decode().splitlines()
                    if line.startswith("data: ")
                ]
                assert (
                    events[0]["type"] == "message_start"
                    and events[-1]["type"] == "message_stop"
                )
                assert any(
                    e.get("content_block", {}).get("name") == "Read" for e in events
                )
                assert events[-2]["delta"]["stop_reason"] == "tool_use"
                result = json.loads(
                    request(
                        {
                            "model": "owned-model",
                            "messages": [
                                {
                                    "role": "user",
                                    "content": [
                                        {
                                            "type": "tool_result",
                                            "tool_use_id": "owned_read_1",
                                            "content": "owned-secret-value",
                                        }
                                    ],
                                }
                            ],
                        }
                    )
                )
                assert (
                    result["stop_reason"] == "end_turn"
                    and "Read allowed" in result["content"][0]["text"]
                )
                assert peer.rows[-1]["secretSeen"] is True
            finally:
                peer.close()
            peer = ModelFixture(root, "error")
            try:
                try:
                    request({"messages": []})
                    raise AssertionError("fixture error was not delivered")
                except urllib.error.HTTPError as error:
                    assert error.code == 400 and b"owned fixture error" in error.read()
            finally:
                peer.close()
        print(
            "Owned model fixture SSE, tool-result continuation and error tests passed"
        )
    else:
        root = Path(sys.argv[1])
        peer = ModelFixture(root, "allow")
        (root / "fixture-port").write_text(str(peer.server.server_port))
        try:
            sys.stdin.readline()
        finally:
            peer.close()
