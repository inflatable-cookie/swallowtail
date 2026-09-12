import csv
import re
import sys
from collections import Counter

with open(sys.argv[1], newline="", encoding="utf-8") as feature_file:
    rows = list(csv.DictReader(feature_file))

if len(rows) != 41:
    raise SystemExit("provider solution feature matrix must contain exactly 41 rows")

providers = [row["provider"] for row in rows]
if providers != sorted(providers, key=str.casefold):
    raise SystemExit("provider solution feature matrix must be sorted by provider")

expected = Counter(
    {
        "Yes": 20,
        "Session-negotiated": 3,
        "Not applicable": 2,
        "Caller-supplied": 16,
    }
)
actual = Counter(row["model_catalog"] for row in rows)
if actual != expected:
    raise SystemExit(
        f"provider solution model_catalog dispositions changed: {dict(actual)}"
    )
