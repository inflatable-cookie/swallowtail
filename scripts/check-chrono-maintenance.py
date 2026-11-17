"""Finite guard for the single pinned chrono maintenance disposition."""

from datetime import date
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
EXPIRY = date(2026, 12, 17)
PINNED = {"chrono": "0.4.45", "cbor-diag": "0.1.12", "serde_with": "3.21.0"}


def check(today, lock):
    if today > EXPIRY:
        raise ValueError("chrono maintenance disposition expired; migrate or review")
    for package, version in PINNED.items():
        versions = re.findall(
            r'name = "' + re.escape(package) + r'"\nversion = "([^"]+)"', lock
        )
        if versions != [version]:
            raise ValueError("chrono disposition dependency changed: " + package)


if __name__ == "__main__":
    lock = (ROOT / "Cargo.lock").read_text()
    if sys.argv[1:] == ["--self-test"]:
        check(EXPIRY, lock)
        for today, text in [
            (date(2026, 12, 18), lock),
            (EXPIRY, lock.replace('version = "0.4.45"', 'version = "0.4.46"')),
        ]:
            try:
                check(today, text)
            except ValueError:
                continue
            raise AssertionError("expired or changed maintenance acceptance passed")
    else:
        check(date.today(), lock)
    print("bounded chrono maintenance disposition passed")
