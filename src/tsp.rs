// TODO

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
    let n = read!(u64);
    let mut xs = vec![];
    let mut ys = vec![];
    for _ in 0..n {
        let (x, y) = read!(f64, f64);
        xs.push(x);
        ys.push(y);
    }
    for i in 0..n {
        println!("{i}");
    }
}
