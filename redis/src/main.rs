extern crate redis;

use redis::{Client, Commands};
mod rdis;
fn main() {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();
    let _: () = conn.set("answer", 42).unwrap();
    rdis::set(&mut conn, "a", "b");
    //let answer:i32= conn.get("answer").unwrap();
    println!("Answer: {}",rdis::get(&mut conn,"a"));
}