extern crate time;
extern crate redis;
use redis::{Client, Commands};
struct Inp{id: String, left: String, right: String, side: String, price: String, amount: u32,}
struct Id{status: String, open: String, history: String, list: String,}
struct Pair{status: String, buy: String, sell: String, history: String,}
struct OpHi{time: String, side: String, pair: String, price: String, amount: String, fill: String, total: String,}
struct List{owned: String, avail: String,}
impl Id{
    // id == id
    fn new(id: &str)-> Id{
        Id{
            status: [id, "status"].join("."),
            open: [id, "open"].join("."),
            history: [id, "history"].join("."),
            list: [id, "list"].join("."),
        }
    }
}
impl Pair{
    // pair == left+right
    fn new(pair: &str)-> Pair{
        Pair{
            status: [pair, "status"].join("."),
            buy: [pair, "buy"].join("."),
            sell: [pair, "sell"].join("."),
            history: [pair, "history"].join("."),
        }
    }
}
impl List{
    // list == left || right
    fn new(list: &str)-> List{
        List{
            owned: [list, "status"].join("."),
            avail: [list, "buy"].join("."),
        }
    }
}
impl OpHi{
    // list == left || right
    fn new(list: &str)-> OpHi{
        OpHi{
            time: [list, "status"].join("."), // mod 7 == 0
            side: [list, "buy"].join("."), // mod 7 == 1
            pair: [list, "status"].join("."), // mod 7 == 2
            price: [list, "buy"].join("."), // mod 7 == 3
            amount: [list, "status"].join("."), // mod 7 == 4
            fill: [list, "buy"].join("."), // mod 7 == 5
            total: [list, "buy"].join("."), // mod 7 == 6
        }
    }
}
mod rdis;
fn main(){
    let _x = time::now_utc();
    let _y = time::get_time();
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();
    rdis::set(&mut conn, "loc", "cho");
    println!("{}",rdis::get(&mut conn,"loc"));
}
