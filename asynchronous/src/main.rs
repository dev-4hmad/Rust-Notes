use futures::{join, stream, StreamExt};
use futures::stream::FuturesUnordered;
use futures::future::BoxFuture;

// 17.1 Futures and the Async Syntax
async fn simple_future() -> i32 {
    42
}

// 17.2 Applying Concurrency with Async
async fn task1() -> &'static str {
    "Task 1 done"
}

async fn task2() -> &'static str {
    "Task 2 done"
}

// 17.3 Working With Any Number of Futures
async fn multiple_futures() {
    let mut tasks: FuturesUnordered<BoxFuture<'static, i32>> = FuturesUnordered::new();

    // Convert each async block into a boxed future so they have the same type
    tasks.push(Box::pin(async { 1 }));
    tasks.push(Box::pin(async { 2 }));
    tasks.push(Box::pin(async { 3 }));

    while let Some(result) = tasks.next().await {
        println!("Got {result}");
    }
}

// 17.4 Streams: Futures in Sequence
async fn stream_example() {
    let mut s = stream::iter(vec![10, 20, 30]);
    while let Some(val) = s.next().await {
        println!("Stream value: {val}");
    }
}

#[tokio::main]
async fn main() {
    // --- 17.1 ---
    let future = simple_future();
    let value = future.await;
    println!("Future resolved with: {value}");

    // --- 17.2 ---
    let (res1, res2) = join!(task1(), task2());
    println!("{res1}, {res2}");

    // --- 17.3 ---
    multiple_futures().await;

    // --- 17.4 ---
    stream_example().await;
}
