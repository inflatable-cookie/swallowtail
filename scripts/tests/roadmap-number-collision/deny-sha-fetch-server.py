#!/usr/bin/env python3
"""Advertise refs/heads/main, then reject fetch-by-SHA over smart HTTP."""

from __future__ import annotations

import argparse
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


FLUSH = b"0000"


def pkt(data: bytes) -> bytes:
    return f"{len(data) + 4:04x}".encode("ascii") + data


def advertisement(sha: bytes) -> bytes:
    caps = (
        b"multi_ack thin-pack side-band side-band-64k ofs-delta shallow "
        b"no-progress include-tag multi_ack_detailed no-done "
        b"symref=HEAD:refs/heads/main agent=git/deny-sha-fetch\n"
    )
    return (
        pkt(b"# service=git-upload-pack\n")
        + FLUSH
        + pkt(sha + b" HEAD\0" + caps)
        + pkt(sha + b" refs/heads/main\n")
        + FLUSH
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--sha", required=True)
    parser.add_argument("--port-file", required=True)
    args = parser.parse_args()
    sha = args.sha.encode("ascii")
    port_file = Path(args.port_file)

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, format: str, *log_args: object) -> None:
            return

        def do_GET(self) -> None:
            if "info/refs" in self.path and "git-upload-pack" in self.path:
                body = advertisement(sha)
                self.send_response(200)
                self.send_header(
                    "Content-Type", "application/x-git-upload-pack-advertisement"
                )
                self.send_header("Cache-Control", "no-cache")
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                self.wfile.write(body)
                return
            self.send_error(404)

        def do_POST(self) -> None:
            length = int(self.headers.get("Content-Length", "0"))
            if length:
                self.rfile.read(length)
            body = pkt(b"ERR fetch by object id denied\n") + FLUSH
            self.send_response(200)
            self.send_header("Content-Type", "application/x-git-upload-pack-result")
            self.send_header("Cache-Control", "no-cache")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

    httpd = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    port_file.write_text(str(httpd.server_address[1]), encoding="ascii")
    httpd.serve_forever()


if __name__ == "__main__":
    main()
