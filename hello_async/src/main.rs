use futures::executor::block_on;
use futures::join;
use futures::future::join_all;

async fn say_hello() {
    println!("Hello, world!");
    join!(second_function(), say_bye());

    let n = double(4).await;
    println!("n: {}", n);
    let futures = vec!{double(1), double(2), double(3)};
    let results = join_all(futures).await;
    println!("results: {:?}", results);
}

async fn second_function() {
    println!("Hello, world again!");
}

async fn say_bye() {
    println!("bye world");
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {
    block_on(say_hello());
}
