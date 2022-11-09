use std::io;

fn main() {
    let mut input: String = String::new();

    println!("generate which fibonacci number? ");
    io::stdin().read_line(&mut input).expect("bozo behaviour");
    println!("input is {input}");

    let input: u64 = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => 0,
    };

    let output: u64 = fibonacci(input);
    println!("returns {output}");
}

fn fibonacci(nth: u64) -> u64 {
    let (mut past, mut current): (u64,u64) = (1,1);
    for _iter in 0..nth {
        (current, past) = (current + past, current);
    };
    current
}