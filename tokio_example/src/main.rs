use tokio::runtime;

async fn hello() {
    println!("Hello, tokio!");
}

// fn main() {
//     // let rt = runtime::Builder::new_current_thread()
//     //     .enable_all()
//     //     .build()
//     //     .unwrap();
//     //
//     // rt.block_on(hello());
//
//     let rt = runtime::Builder::new_multi_thread()
//         .enable_all()
//         .worker_threads(4)
//         .build()
//         .unwrap();
//
//     rt.block_on(hello());
// }

#[tokio::main(flavor = "current_thread")]
async fn main() {
    hello().await;
}
