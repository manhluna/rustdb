extern crate redis;
use redis::{Commands, Connection};
//-----------------------------------------------key-value
//String
pub fn set(conn: &mut Connection,key: &str, value: &str){
    let _: () = conn.set(key, value).unwrap();
}
pub fn set_ex(conn:&mut Connection,key:&str,value:&str,seconds:usize){
    let _:() = conn.set_ex(key,value,seconds).unwrap();
}
pub fn del(conn:&mut Connection,key:&str){
    let _:() = conn.del(key).unwrap();
}
pub fn incr(conn:&mut Connection,key:&str,delta:usize){
    let _:() = conn.incr(key,delta).unwrap();
}
pub fn get(conn:&mut Connection,key:&str)->String{
    conn.get(key).unwrap()
}
//-----------------------------------------------hash
pub fn hset(conn:&mut Connection,key:&str,field:&str,value:&str){ //--> Them field vao hash
    let _:() = conn.hset(key,field,value).unwrap();
}
pub fn hincr(conn:&mut Connection,key:&str,field:&str,delta:usize){
    let _:() = conn.hincr(key,field,delta).unwrap();
}
pub fn hdel(conn:&mut Connection,key:&str,field:&str){ //--> xoa field cua hash
    let _:() = conn.hdel(key,field).unwrap();
}
pub fn hget(conn:&mut Connection,key:&str,field:&str)->String{ //--> Tra ve value cua 1 field
    conn.hget(key,field).unwrap()
}
pub fn hvals(conn:&mut Connection,key:&str) -> Vec<(String)> {
    conn.hvals(key).unwrap()
}
pub fn hgetall(conn:&mut Connection,key:&str)-> Vec<(String,String)>{ //--> Tra ve tap (field,value) cua hash
    conn.hgetall(key).unwrap()
}
//-----------------------------------------------sorted set
pub fn zadd(conn:&mut Connection,key:&str,member:&str,score:usize){ //--> Them member vao sorted set
    let _:() = conn.zadd(key,member,score).unwrap();
}
pub fn zincr(conn:&mut Connection,key:&str,member:&str,delta:usize){
    let _:() = conn.zincr(key,member,delta).unwrap();
}
pub fn zrem(conn:&mut Connection,key:&str,member: &str){ //--> xoa member khoi sorted set
    let _:() = conn.zrem(key,member).unwrap();
}
pub fn zrange(conn:&mut Connection,key:&str,start:isize,stop:isize)-> Vec<String>{ //--> tra ve tap member theo khoang index
    conn.zrange(key,start,stop).unwrap()
}
pub fn zrange_withscores(conn:&mut Connection,key:&str,start:isize,stop:isize)-> Vec<(String,usize)>{ //--> tra ve tap (member,score) theo khoang index
    conn.zrange_withscores(key,start,stop).unwrap()
}
pub fn zrangebyscore(conn:&mut Connection,key:&str,min:usize,max:usize)-> Vec<String>{ //--> tra ve tap member theo khoang score
    conn.zrangebyscore(key,min,max).unwrap()
}
pub fn zrangebyscore_withscores(conn:&mut Connection,key:&str,min:usize,max:usize)-> Vec<(String,usize)>{ //--> tra ve tap (member,score) theo khoang score
    conn.zrangebyscore_withscores(key,min,max).unwrap()
}
pub fn zcard(conn:&mut Connection,key:&str)-> isize{ //--> Tra ve do dai cua sorted set
    conn.zcard(key).unwrap()
}