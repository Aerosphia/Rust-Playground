fn main() {
    let res: String = hello().await;
}

async fn hello() {
    println!("Hello, world!");
}
