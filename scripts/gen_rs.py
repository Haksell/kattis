import os
import re
import sys

CONFIG_FILE = "Cargo.toml"

assert len(sys.argv) == 2
executable = sys.argv[1]
assert re.fullmatch(r"\w+", executable)
filename = f"src/{executable}.rs"

toml = open(CONFIG_FILE).read()
assert filename not in toml

bin_idx = toml.find("[[bin]]")
start = toml[:bin_idx]

bins = list(map(str.strip, toml[bin_idx:].split("\n\n")))
bins.append(f'''[[bin]]
name = "{executable}"
path = "{filename}"''')
bins.sort()

print(start + "\n\n".join(bins), file=open(CONFIG_FILE, "w"))
os.makedirs(os.path.dirname(filename), exist_ok=True)
print("fn main() {\n    \n}", file=open(filename, "w"))
os.system(f"code {filename}")
