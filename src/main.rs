use multithreading::{converting, multi_thread, multi_thread_rayon, multi_thread_tokio};
// #[tokio::main]
// async fn main() {
//     let numbers = vec![1, 2, 3, 100];
//     let result = multi_thread_tokio(numbers, converting).await;
//     println!("{:?}", result);
// }
fn main() {
    let numbers = vec![1, 2, 3, 100];
    let result = multi_thread(numbers, converting);
    println!("{:?}", result);
}
