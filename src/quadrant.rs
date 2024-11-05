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
    let x = read!(i32);
    let y = read!(i32);
    println!(
        "{}",
        if y > 0 {
            if x > 0 {
                1
            } else {
                2
            }
        } else {
            if x < 0 {
                3
            } else {
                4
            }
        }
    );
}
