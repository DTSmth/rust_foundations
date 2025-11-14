fn test() {
    println!("test");
}

fn main() {
    // Explicitly sized pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.join(test, test);

    // pool.scope(|scope| {
    //     scope.spawn_broadcast(|_scope, broadcast_context| {
    //        println!("broadcast thread {}", broadcast_context.index());
    //     });
    // });

    // pool.spawn(|| println!("Hello, world from pool thread!"));
    //
    // pool.scope(|scope| {
    //    for n in 0..10 {
    //        scope.spawn(move |_| {
    //            println!("Hello, world from scope thread! {n}");
    //        })
    //    }
    // });
    //
    // println!("Hello, world from main thread!");


}
