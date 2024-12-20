import os
import re
import sys

CONFIG_FILE = "Cargo.toml"
TEMPLATE = """\
macro_rules! read {
    ( $t:ty ) => {
        {
            let stdin = std::io::stdin();
            let mut line = String::new();
            stdin.read_line(&mut line)
                .expect("Failed to read line");
            line.trim()
                .parse::<$t>()
                .expect("Failed to parse input")
        }
    };
    ( $( $t:ty ),+ ) => {
        {
            let stdin = std::io::stdin();
            let mut line = String::new();
            stdin.read_line(&mut line)
                .expect("Failed to read line");
            let mut iter = line.trim().split_whitespace();
            (
                $(
                    iter.next()
                        .expect("Not enough input values")
                        .parse::<$t>()
                        .expect("Failed to parse input"),
                )+
            )
        }
    };
}

fn main() {

}
"""

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
print(TEMPLATE, file=open(filename, "w"))
os.system(f"code {filename}")
