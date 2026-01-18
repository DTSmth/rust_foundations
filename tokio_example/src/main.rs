async fn hello() -> u32 {
    println!("Hello, tokio!");
    3
}

async fn hello2() -> u32 {
    println!("hello world2");
    4
}

async fn ticker() {
    for i in 0 .. 10 {
        println!("tick {}", i);
        tokio::task::yield_now().await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // hello().await;

    // let result = tokio::join!(hello2(), hello());
    // println!("result: {:?}", result);
    // let (one, two) = result;
    tokio::spawn(ticker());
    hello().await;

    let _ = tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(ticker())
    );
    println!("done");
}
