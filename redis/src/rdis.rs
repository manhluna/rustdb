extern crate redis;

use redis::{Commands, Connection};
pub fn set(conn: &mut Connection,key: &str, value: &str){
    let _: () = conn.set(key, value).unwrap();
}
pub fn get(conn: &mut Connection,key: &str) -> String{
    conn.get(key).unwrap()
}