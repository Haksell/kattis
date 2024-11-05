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
    for i in 1..=read!(u8) {
        println!("{i} Abracadabra");
    }
}
