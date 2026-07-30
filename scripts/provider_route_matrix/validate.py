from pathlib import Path

fragment_root = Path(__file__).parent
namespace = {"__name__": "__main__"}

for fragment_name in [
    "base.py",
    "route_records.py",
    "inventory.py",
    "classification_inputs.py",
    "classifications.py",
    "assertions.py",
]:
    fragment = fragment_root / fragment_name
    source = fragment.read_text(encoding="utf-8")
    exec(compile(source, str(fragment), "exec"), namespace)
