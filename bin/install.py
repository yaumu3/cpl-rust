#!/usr/bin/python3
# Installs VSCode styled snippets with `extra_snippets.json` concatenated

import json
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent
EXTRA_SNIPS = REPO_ROOT / "extra_snippets.json"

try:
    dest = Path(sys.argv[1])
except IndexError:
    print("Usage: ./install.py DEST_FILE", file=sys.stderr)
    sys.exit(1)

OVR_CFM = "Are you sure to overwrite existing file? [Y/n]: "
if dest.is_dir():
    raise IsADirectoryError()
if dest.is_file() and input(OVR_CFM).lower() != "y":
    sys.exit(0)

snips = {}
with open(EXTRA_SNIPS) as fp:
    snips = json.load(fp)

proc = subprocess.run(
    ["cargo", "snippet", "-t", "vscode"],
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    cwd=REPO_ROOT,
)
if proc.returncode != 0:
    print(proc.stderr.decode("utf-8"), file=sys.stderr)
    sys.exit(1)

snips = {**snips, **json.loads(proc.stdout.decode("utf-8"))}
with open(dest, mode="w") as fp:
    json.dump(snips, fp, ensure_ascii=False, indent=2)
print("Done.")