use std::{thread, time};

fn main() {
    
}

async fn future_test() {
    let value: u8 = get_value().await;
    println!("{value}");
}

async fn get_value() -> u8 {
    thread::sleep(time::Duration::from_secs(1));
    5
}
