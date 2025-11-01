fn double(n: i32) -> i32 {
    n * 2
}

fn greet(s: String) {
    println!("{}", s);
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

fn main() {
    println!("type something");
    let input = read_line();
    println!("you typed : {}", input);
}
