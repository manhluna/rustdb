extern crate chrono;
extern crate redis;
use chrono::prelude::*;
use redis::{Commands, Connection};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::cmp::max;
use std::f64::MAX;

//Struct
struct Pair{
    global: String, //amountbuy, amountsell,last,change,vol,high,low
    buy: String,
    sell: String,
    history: String,
    indicator: String,
}
impl Pair{
    fn new(asset:&str,unit:&str)->Pair{
        let pair = &key(&[asset,":",unit,":"]);
        Pair{
            global: key(&[pair,"global"]),
            buy: key(&[pair,"buybook"]),
            sell: key(&[pair,"sellbook"]),
            history: key(&[pair,"history"]),
            indicator: key(&[pair,"indicator"]),
        }
    }
}
struct Phone{
    myorder: String,
    myhistory: String,
    myassets: String,
}
impl Phone{
    fn new(phone:&str)->Phone{
        Phone{
            myorder: key(&[phone,":","myorder"]),
            myhistory: key(&[phone,":","myhistory"]),
            myassets: key(&[phone,":","myassets"]),
        }
    }
}
struct MyOrder{
    price: String,
    time: String,
    side: String,
    pair: String,
    qrice: String,
    amount: String,
    fill: String,
    total: String,
}
impl MyOrder{
    fn new(asset:&str,unit:&str,price:f64)->MyOrder{
        let cost = &key(&[asset,":",unit,":",&tostring(price),":"]);
        MyOrder{
            price: key(&[asset,":",unit,":",&tostring(price)]),
            time: key(&[cost,"time"]),
            side: key(&[cost,"side"]),
            pair: key(&[cost,"pair"]),
            qrice: key(&[cost,"price"]),
            amount: key(&[cost,"amount"]),
            fill: key(&[cost,"fill"]),
            total: key(&[cost,"total"]),
        }
    }
    fn newhistory(timestamp:&str)-> MyOrder{
        MyOrder{
            price: String::new(),
            time: key(&[timestamp,":time"]),
            side: key(&[timestamp,":side"]),
            pair: key(&[timestamp,":pair"]),
            qrice: key(&[timestamp,":price"]),
            amount: key(&[timestamp,":amount"]),
            fill: key(&[timestamp,":fill"]),
            total: key(&[timestamp,":total"]),
        }
    }
}
struct Indi{
    side: String,
    delta_time: String,
    pre_time: String,
    avprice: String,
    avvol: String,
    totalprice: String,
    totalvol: String,
    slg: String,
}
impl Indi{
    fn new(timestamp:&str) -> Indi {
        Indi{
            side: key(&[timestamp,":side"]),
            delta_time: key(&[timestamp,":delta_time"]),
            avprice: key(&[timestamp,":avprice"]),
            avvol: key(&[timestamp,":avvol"]),
            pre_time: key(&["pre_time"]),
            totalprice: key(&["toltalprice"]),
            totalvol: key(&["totalvol"]),
            slg: key(&["slg"]),
        }
    }
}
struct MyAssets{
    logolnk: String,
    symbol: String,
    name: String,
    owned: String,
    available: String,
    deposit: String,
}
impl MyAssets{
    fn new(asset:&str) -> MyAssets{
        MyAssets{
            logolnk: key(&[asset,":","logolnk"]),
            symbol: key(&[asset,":","symbol"]),
            name: key(&[asset,":","name"]),
            owned: key(&[asset,":","owned"]),
            available: key(&[asset,":","available"]),
            deposit: key(&[asset,":","deposit"]),
        }
    }
}
fn key(para:&[&str])->String{
    let mut s=String::from("");
    for i in para.iter() {
        s.push_str(i);
    }
    s
}
fn unix() -> u128 { //lay timestamp
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
}
fn ymd(unix:u128)->String{ //lay ngaythang
    //let ms=unix%1000;
    let s=(unix/1000)%60;
    let m=(unix/60000)%60;
    let h=(unix/3600000%24);
    let d=Utc::today();
    format!("{} {}:{}:{}",&d.to_string()[0..10],h.to_string(),m.to_string(),s.to_string())
}
fn hms(unix:u128)->String{ //lay gio phut
    let s=(unix/1000)%60;
    let m=(unix/60000)%60;
    let h=(unix/3600000%24);
    format!("{}:{}:{}",h.to_string(),m.to_string(),s.to_string())
}
fn tof64(s:&str)->f64{
    s.parse::<f64>().unwrap()
}
fn tostring(s:f64)->String{
    s.to_string()
}
fn tostrings(s:u128)->String{
    s.to_string()
}
fn s(i:&str) -> String{ //chuyen &str sang String
    String::from(i)
}
//Redis
//-----------------------------------------------key-value
//String
fn set(con:&Connection,key:&str,value:&str) -> redis::RedisResult<()> {
    let _:() = con.set(key,value)?;
    Ok(())
}
fn set_ex(con:&Connection,key:&str,value:&str,seconds:usize) -> redis::RedisResult<()> {
    let _:() = con.set_ex(key,value,seconds)?;
    Ok(())
}
fn get(con:&Connection,key:&str)->String{
    match con.get(key) {
        Ok(i) => i,
        _ => String::new(),
    }
}
fn del(con:&Connection,key:&str) -> redis::RedisResult<()> {
    let _:() = con.del(key)?;
    Ok(())
}
fn incr(con:&Connection,key:&str,delta:f64)-> redis::RedisResult<()>{
    let _:() = con.incr(key,delta)?;
    Ok(())
}
//-----------------------------------------------hash
fn hset(con:&Connection,key:&str,field:&str,value:&str)-> redis::RedisResult<()>{ //--> Them field vao hash
    let _:() = con.hset(key,field,value)?;
    Ok(())
}
fn hget(con:&Connection,key:&str,field:&str)->String{ //--> Tra ve value cua 1 field
    match con.hget(key,field) {
        Ok(i) => i,
        _ =>String::new(),
    }
}
fn hvals(con:&Connection,key:&str) -> Vec<(String)> {
    match con.hvals(key) {
        Ok(i) => i,
        _ => Vec::new(),
    }
}
fn hgetall(con:&Connection,key:&str)-> Vec<(String,String)>{ //--> Tra ve tap (field,value) cua hash
    match con.hgetall(key) {
        Ok(i) => i,
        _ => Vec::new(),
    }
}
fn hincr(con:&Connection,key:&str,field:&str,delta:f64) ->redis::RedisResult<()> {
    let _:() = con.hincr(key,field,delta)?;
    Ok(())
}
fn hdel(con:&Connection,key:&str,field:&str)-> redis::RedisResult<()>{ //--> xoa field cua hash
    let _:() = con.hdel(key,field)?;
    Ok(())
}
//-----------------------------------------------sorted set
fn zadd(con:&Connection,key:&str,member:&str,score:f64)-> redis::RedisResult<()>{ //--> Them member vao sorted set
    let _:() = con.zadd(key,member,score)?;
    Ok(())
}
fn zrange(con:&Connection,key:&str,start:isize,stop:isize)-> Vec<String>{ //--> tra ve tap member theo khoang index
    match con.zrange(key,start,stop) {
        Ok(i) => i,
        _ => Vec::new(),
    }
}
fn zrange_withscores(con:&Connection,key:&str,start:isize,stop:isize)-> Vec<(String,f64)>{ //--> tra ve tap (member,score) theo khoang index
    match con.zrange_withscores(key,start,stop) {
        Ok(i) => i,
        _ => Vec::new(),
    }
}
fn zrangebyscore(con:&Connection,key:&str,min:f64,max:f64)-> Vec<String>{ //--> tra ve tap member theo khoang score
    match con.zrangebyscore(key,min,max) {
        Ok(i) => i,
        _ => Vec::new(),
    }
}
fn zrangebyscore_withscores(con:&Connection,key:&str,min:f64,max:f64)-> Vec<(String,f64)>{ //--> tra ve tap (member,score) theo khoang score
    match con.zrangebyscore_withscores(key,min,max) {
        Ok(i) => i,
        _ => Vec::new(),
    }
}
fn zincr(con:&Connection,key:&str,member:&str,delta:f64)-> redis::RedisResult<()>{
    let _:() = con.zincr(key,member,delta)?;
    Ok(())
}
fn zrem(con:&Connection,key:&str,member: &str)-> redis::RedisResult<()>{ //--> xoa member khoi sorted set
    let _:() = con.zrem(key,member)?;
    Ok(())
}
fn zcard(con:&Connection,key:&str)-> isize{ //--> Tra ve do dai cua sorted set
    match con.zcard(key) {
        Ok(i) => i,
        _ => -1,
    }
}
//-------------------------------engine
fn genesis(r:&Connection,asset:&str,unit:&str){
    let pair = Pair::new(asset,unit);
    let ts = unix();
    let timestamp = &tostrings(ts);
    let indicator = Indi::new(timestamp);
    hset(r,&pair.indicator,&indicator.pre_time,timestamp);
    zadd(r,&pair.sell,"MAX",MAX);
    zadd(r,&pair.buy,"MIN",0.0);
}
fn handle_limit_basic(r:&Connection,myorder:MyOrder,ph0ne:Phone,phone:&str,price:f64,amount:f64,v:(&str,&String,&str,&String,f64,&String,f64),asset:&str,unit:&str){
    let ts = unix();
    let pr = &key(&[asset,"-",unit]);
    hincr(r,v.5,v.0,v.6); //them khoi luong vao tong
    zadd(r,v.1,&myorder.price,price); //Them gia vao buybook

    zincr(r,&myorder.price,&myorder.price,amount); //Tang amount o dau queue gia || index 0
    zadd(r,&myorder.price,phone,ts as f64); //Them nguoi mua vao queue || index 1 ->

    hset(r,&ph0ne.myorder,&myorder.time,&ymd(ts)); //them time vao danh sach lenh
    hset(r,&ph0ne.myorder,&myorder.side,v.2); //them side vao danh sach lenh
    hset(r,&ph0ne.myorder,&myorder.pair,pr); //them side vao danh sach lenh
    hset(r,&ph0ne.myorder,&myorder.qrice,&tostring(price)); //them side vao danh sach lenh
    hincr(r,&ph0ne.myorder,&myorder.amount,amount); //them amount vao danh sach lenh
    hincr(r,&ph0ne.myorder,&myorder.fill,0.0); //them amount vao danh sach lenh
    hincr(r,&ph0ne.myorder,&myorder.total,amount*price);

    hincr(r,&ph0ne.myassets,v.3,v.4); //tru so tien kha dung trong tai khoan
}
fn add_history(r:&Connection,ph0ne:&Phone,myhistory:MyOrder,myorder:&MyOrder){
    hset(r,&ph0ne.myhistory,&myhistory.time,&ymd(unix()));
    hset(r,&ph0ne.myhistory,&myhistory.side,&hget(r,&ph0ne.myorder,&myorder.side));
    hset(r,&ph0ne.myhistory,&myhistory.pair,&hget(r,&ph0ne.myorder,&myorder.pair));
    hset(r,&ph0ne.myhistory,&myhistory.amount,&hget(r,&ph0ne.myorder,&myorder.amount));
    hset(r,&ph0ne.myhistory,&myhistory.qrice,&hget(r,&ph0ne.myorder,&myorder.qrice));
    hset(r,&ph0ne.myhistory,&myhistory.fill,&hget(r,&ph0ne.myorder,&myorder.fill));
    hset(r,&ph0ne.myhistory,&myhistory.total,&hget(r,&ph0ne.myorder,&myorder.total));
}
fn delete_order(r:&Connection,ph0ne:&Phone,myorder:&MyOrder){
    hdel(r,&ph0ne.myorder,&myorder.time);
    hdel(r,&ph0ne.myorder,&myorder.side);
    hdel(r,&ph0ne.myorder,&myorder.pair);
    hdel(r,&ph0ne.myorder,&myorder.amount);
    hdel(r,&ph0ne.myorder,&myorder.qrice);
    hdel(r,&ph0ne.myorder,&myorder.fill);
    hdel(r,&ph0ne.myorder,&myorder.total);
}
fn event_limit_buy(r:&Connection,phone:&str,asset:&str,unit:&str,price:f64,amount:f64){
    let pair = Pair::new(asset,unit);
    let myorder = MyOrder::new(asset,unit,price);
    let ph0ne = Phone::new(phone);
    let myunits = MyAssets::new(unit);
    let v = ("amountbuy",&pair.buy,"BUY",&myunits.available,-price*amount,&pair.global,amount);
    let ask = zrange_withscores(r,&pair.sell,0,0)[0].1;
    if price<ask {
        //limit
        handle_limit_basic(r,myorder,ph0ne,phone,price,amount,v,asset,unit);
    } else {
        //Taker
        let mut ton = 0.0;
        let mut so = 0.0;
        let region = zrangebyscore_withscores(r,&pair.sell,ask,price);
        for i in &region {
            let j = zrange_withscores(r,&i.0,0,0)[0].1;
            so +=j;
            ton +=j*i.1;
        }
        if amount <= so {
            //goi lenh market cho amount
            event_market_buy(r,phone,asset,unit,amount);
        } else {
            let am = amount - so;
            event_market_buy(r,phone,asset,unit,ton);
            event_limit_buy(r,phone,asset,unit,price,am);
        }
    }
}
fn event_limit_sell(r:&Connection,phone:&str,asset:&str,unit:&str,price:f64,amount:f64){
    let pair = Pair::new(asset,unit);
    let myorder = MyOrder::new(asset,unit,price);
    let ph0ne = Phone::new(phone);
    let myassets = MyAssets::new(asset);
    let v = ("amountsell",&pair.sell,"SELL",&myassets.available,-amount,&pair.global,amount*price);
    let bid = zrange_withscores(r,&pair.buy,0,0)[0].1;
    if price>bid {
        //limit
        handle_limit_basic(r,myorder,ph0ne,phone,price,amount,v,asset,unit);
    } else {
        //Taker
        let mut ton = 0.0;
        let mut so = 0.0;
        let region = zrangebyscore(r,&pair.buy,price,bid);
        for i in &region {
            let j = zrange_withscores(r,i,0,0)[0].1;
            so +=j;
        }
        if amount <= so {
            //goi lenh market cho amount
            event_market_sell(r,phone,asset,unit,amount);
        } else {
            let am = amount - so;
            event_market_sell(r,phone,asset,unit,so);
            event_limit_sell(r,phone,asset,unit,price,am);
        }
    }
}
fn event_market_sell(r:&Connection,phone:&str,asset:&str,unit:&str,amount:f64){
    let pair = Pair::new(asset,unit);
    let vbuy = tof64(&hvals(r,&pair.global)[1]);
    if amount <=vbuy {
        //market
        hincr(r,&pair.global,"amountbuy",-amount); //giam tong.
        let mut s = amount;
        let mut cong = 0.0; //cong tong
        let ilen = zcard(r,&pair.buy);
        'price: for i in (1..ilen).rev() {
            let iprice=zrange_withscores(r,&pair.buy,i,i)[0].1; //
            let myorder = MyOrder::new(asset,unit,iprice);
            let jlen = zcard(r,&myorder.price);
            let myassets = MyAssets::new(asset);
            let myunits = MyAssets::new(unit);
            'phone: for j in 1..jlen {
                let jphone=&zrange(r,&myorder.price,j,j)[0];
                let ph0ne = Phone::new(jphone);
                let jasset_amount = hget(r,&ph0ne.myorder,&myorder.amount); //
                let jasset_fill = hget(r,&ph0ne.myorder,&myorder.fill);
                let cl = tof64(&jasset_amount) - tof64(&jasset_fill);
                if s>= cl {
                    hincr(r,&ph0ne.myassets,&myassets.owned,cl);
                    hincr(r,&ph0ne.myassets,&myassets.available,cl);
                    hincr(r,&ph0ne.myassets,&myunits.owned,-cl*iprice);
                    hincr(r,&ph0ne.myorder,&myorder.fill,cl);
                    zincr(r,&myorder.price,&myorder.price,-cl);
                    cong+=cl*iprice;

                    let timestamp = &tostrings(unix());
                    let myhistory = MyOrder::newhistory(timestamp);
                    add_history(r,&ph0ne,myhistory,&myorder);
                    delete_order(r,&ph0ne,&myorder);
                    s-=cl;
                } else {
                    //Nguoi duoc khop cuoi cung
                    hincr(r,&ph0ne.myassets,&myassets.owned,s);
                    hincr(r,&ph0ne.myassets,&myassets.available,s);//?????
                    hincr(r,&ph0ne.myassets,&myunits.owned,-s*iprice);
                    hincr(r,&ph0ne.myorder,&myorder.fill,s);
                    zincr(r,&myorder.price,&myorder.price,-s);
                    cong+=s*iprice;
                    //Xu ly xoa gia
                    let sav = zrange(r,&pair.buy,i+1,ilen-1);
                    for ss in &sav {
                        zrem(r,&pair.buy,ss);
                        del(r,ss);
                    }
                    let sav = zrange(r,&myorder.price,1,j-1);
                    let pp = zrange_withscores(r,&myorder.price,0,0)[0].1;
                    if pp == 0.0 {
                        zrem(r,&pair.buy,&myorder.price);
                        del(r,&myorder.price);
                    } else {
                        for ss in &sav {
                            zrem(r,&myorder.price,ss);
                        }
                    }
                    // xu ly cho ng market
                    let ph0ne = Phone::new(phone);
                    hincr(r,&ph0ne.myassets,&myassets.owned,-amount);
                    hincr(r,&ph0ne.myassets,&myunits.owned,cong);
                    hincr(r,&ph0ne.myassets,&myunits.available,cong);

                    //myHistory for market
                    let avg_price = cong/amount;
                    let pr = &key(&[asset,"-",unit]);
                    let ts = unix();
                    let timestamp = &tostrings(ts);
                    let myhistory = MyOrder::newhistory(timestamp);

                    hset(r,&ph0ne.myhistory,&myhistory.time,&ymd(ts));
                    hset(r,&ph0ne.myhistory,&myhistory.side,"SELL");
                    hset(r,&ph0ne.myhistory,&myhistory.pair,pr);
                    hset(r,&ph0ne.myhistory,&myhistory.amount,&tostring(amount));
                    hset(r,&ph0ne.myhistory,&myhistory.qrice,&tostring(avg_price));
                    hset(r,&ph0ne.myhistory,&myhistory.fill,&tostring(amount));
                    hset(r,&ph0ne.myhistory,&myhistory.total,&tostring(cong));
                    //History Global
                    hset(r,&pair.history,&myhistory.side,"SELL");
                    hset(r,&pair.history,&myhistory.time,&hms(ts));
                    hset(r,&pair.history,&myhistory.price,&tostring(avg_price));
                    hset(r,&pair.history,&myhistory.amount,&tostring(amount));
                    //Indicator
                    let indicator = Indi::new(timestamp);
                    hincr(r,&pair.indicator,&indicator.slg,1.0);
                    hincr(r,&pair.indicator,&indicator.totalvol,amount);
                    hincr(r,&pair.indicator,&indicator.totalprice,avg_price);
                    hset(r,&pair.indicator,&indicator.side,"SELL");

                    let sl = tof64(&hget(r,&pair.indicator,&indicator.slg));
                    let tv = tof64(&hget(r,&pair.indicator,&indicator.totalvol));
                    let tp = tof64(&hget(r,&pair.indicator,&indicator.totalprice));
                    let pre = tof64(&hget(r,&pair.indicator,&indicator.pre_time));
                    let det = ts as f64 - pre;

                    hset(r,&pair.indicator,&indicator.avprice,&tostring(tp/sl));
                    hset(r,&pair.indicator,&indicator.avvol,&tostring(tv/sl));
                    hincr(r,&pair.indicator,&indicator.delta_time,det);
                    hset(r,&pair.indicator,&indicator.side,"BUY");

                    hset(r,&pair.indicator,&indicator.pre_time,timestamp);
                    //----chi so
                    hset(r,&pair.global,"last",&tostring(avg_price));
                    let call_history = hvals(r,&pair.history);

                    println!("{}",call_history.len()/4);

                    let mut soluong:usize =0;
                    let mut z:usize = 0;
                    if call_history.len()/4 <= 100 {
                        soluong = call_history.len()/4;
                        println!("{}",soluong);
                        z = 2;
                    } else {
                        soluong = 100;
                        z = 2+(call_history.len()-400);
                    }
                    let mut dem =1;
                    let mut vol100=0.0;
                    let mut max_price100 = 0.0;
                    let mut min_price100 = MAX;

                    println!("{}",soluong);

                    while dem <= soluong {
                        let temp_price = tof64(&call_history[z]);
                        if temp_price > max_price100 {
                            max_price100 = temp_price;
                        }
                        if temp_price < min_price100 {
                            min_price100 = temp_price;
                        }
                        if dem == 1 {
                            let last = &tof64(&hget(r,&pair.global,"last"));
                            let change:f64 = (last/temp_price - 1.0)*100.0;
                            hset(r,&pair.global,"change",&tostring(change));
                        }
                        let temp_vol = tof64(&call_history[z+1]);
                        vol100+=temp_vol;
                        z+=4;
                        dem+=1;
                    }
                    hset(r,&pair.global,"high",&tostring(max_price100));
                    hset(r,&pair.global,"low",&tostring(min_price100));
                    hset(r,&pair.global,"vol",&tostring(vol100));
                    break 'price;
                    break 'phone;
                }
            }
        }
    }
}
fn event_market_buy(r:&Connection,phone:&str,asset:&str,unit:&str,amount:f64){
    let pair = Pair::new(asset,unit);
    let vsell = tof64(&hvals(r,&pair.global)[0]);  //vsell la tong tien theo unit
    if amount <=vsell {
        //market
        hincr(r,&pair.global,"amountsell",-amount); //giam tong.
        let mut s = amount;
        let mut cong = 0.0; //cong tong
        let ilen = zcard(r,&pair.sell)-1;
        'price: for i in 0..ilen {
            let iprice=zrange_withscores(r,&pair.sell,i,i)[0].1; //
            let myorder = MyOrder::new(asset,unit,iprice);
            let jlen = zcard(r,&myorder.price);
            let myassets = MyAssets::new(asset);
            let myunits = MyAssets::new(unit);
            'phone: for j in 1..jlen {
                let jphone=&zrange(r,&myorder.price,j,j)[0];
                let ph0ne = Phone::new(jphone);
                let jasset_amount = hget(r,&ph0ne.myorder,&myorder.amount); //
                let jasset_fill = hget(r,&ph0ne.myorder,&myorder.fill);
                let cl = tof64(&jasset_amount) - tof64(&jasset_fill);
                if s>= cl*iprice {
                    hincr(r,&ph0ne.myassets,&myassets.owned,-cl);
                    hincr(r,&ph0ne.myassets,&myunits.owned,cl*iprice);
                    hincr(r,&ph0ne.myassets,&myunits.available,cl*iprice);//?????????
                    hincr(r,&ph0ne.myorder,&myorder.fill,cl);
                    zincr(r,&myorder.price,&myorder.price,-cl);
                    cong+=cl;

                    let timestamp = &tostrings(unix());
                    let myhistory = MyOrder::newhistory(timestamp);
                    add_history(r,&ph0ne,myhistory,&myorder);
                    delete_order(r,&ph0ne,&myorder);
                    s-=cl*iprice;
                } else {
                    //Nguoi duoc khop cuoi cung
                    hincr(r,&ph0ne.myassets,&myassets.owned,-s/iprice);
                    hincr(r,&ph0ne.myassets,&myunits.owned,s);
                    hincr(r,&ph0ne.myassets,&myunits.available,s);
                    hincr(r,&ph0ne.myorder,&myorder.fill,s/iprice);
                    zincr(r,&myorder.price,&myorder.price,-s/iprice);
                    cong+=s/iprice;
                    //xu ly xoa gia
                    let sav = zrange(r,&myorder.price,1,j-1);
                    let pp = zrange_withscores(r,&myorder.price,0,0)[0].1;
                    if pp == 0.0 {
                        zrem(r,&pair.sell,&myorder.price);
                        del(r,&myorder.price);
                    } else {
                        for ss in &sav {
                            zrem(r,&myorder.price,ss);
                        }
                    }

                    let sav = zrange(r,&pair.sell,0,i-1);
                    for ss in &sav {
                        zrem(r,&pair.sell,ss);
                        del(r,ss);
                    }
                    // xu ly cho ng market
                    let ph0ne = Phone::new(phone);
                    hincr(r,&ph0ne.myassets,&myassets.owned,cong);
                    hincr(r,&ph0ne.myassets,&myassets.available,cong);
                    hincr(r,&ph0ne.myassets,&myunits.owned,-amount);
                    //myHistory for market
                    let avg_price = amount/cong;
                    let pr = &key(&[asset,"-",unit]);
                    let ts = unix();
                    let timestamp = &tostrings(ts);
                    let myhistory = MyOrder::newhistory(timestamp);

                    hset(r,&ph0ne.myhistory,&myhistory.time,&ymd(ts));
                    hset(r,&ph0ne.myhistory,&myhistory.side,"BUY");
                    hset(r,&ph0ne.myhistory,&myhistory.pair,pr);
                    hset(r,&ph0ne.myhistory,&myhistory.amount,&tostring(amount));
                    hset(r,&ph0ne.myhistory,&myhistory.qrice,&tostring(avg_price));
                    hset(r,&ph0ne.myhistory,&myhistory.fill,&tostring(amount));
                    hset(r,&ph0ne.myhistory,&myhistory.total,&tostring(cong));
                    //History Global
                    hset(r,&pair.history,&myhistory.side,"BUY");
                    hset(r,&pair.history,&myhistory.time,&hms(ts));
                    hset(r,&pair.history,&myhistory.price,&tostring(avg_price));
                    hset(r,&pair.history,&myhistory.amount,&tostring(amount));
                    //Indicator
                    let indicator = Indi::new(timestamp);
                    hincr(r,&pair.indicator,&indicator.slg,1.0);
                    hincr(r,&pair.indicator,&indicator.totalvol,cong);
                    hincr(r,&pair.indicator,&indicator.totalprice,avg_price);

                    let sl = tof64(&hget(r,&pair.indicator,&indicator.slg));
                    let tv = tof64(&hget(r,&pair.indicator,&indicator.totalvol));
                    let tp = tof64(&hget(r,&pair.indicator,&indicator.totalprice));
                    let pre = tof64(&hget(r,&pair.indicator,&indicator.pre_time));
                    let det = ts as f64 - pre;

                    hset(r,&pair.indicator,&indicator.avprice,&tostring(tp/sl));
                    hset(r,&pair.indicator,&indicator.avvol,&tostring(tv/sl));
                    hset(r,&pair.indicator,&indicator.delta_time,&tostring(det));
                    hset(r,&pair.indicator,&indicator.side,"BUY");

                    hset(r,&pair.indicator,&indicator.pre_time,timestamp);
                    //----chi so
                    hset(r,&pair.global,"last",&tostring(avg_price));
                    let call_history = hvals(r,&pair.history);
                    let soluong:usize =0;
                    let mut z:usize = 0;
                    if call_history.len()/4 <= 100 {
                        let soluong = call_history.len()/4;
                        let mut z = 2;
                    } else {
                        let soluong = 100;
                        let mut z = 2+(call_history.len()-400);
                    }
                    let mut dem =1;
                    let mut vol100=0.0;
                    let mut max_price100 = 0.0;
                    let mut min_price100 = MAX;
                    while dem <= soluong {
                        let temp_price = tof64(&call_history[z]);
                        if temp_price > max_price100 {
                            max_price100 = temp_price;
                        }
                        if temp_price < min_price100 {
                            min_price100 = temp_price;
                        }
                        if dem == 1 {
                            let last = &tof64(&hget(r,&pair.global,"last"));
                            let change:f64 = (last/temp_price - 1.0)*100.0;
                            hset(r,&pair.global,"change",&tostring(change));
                        }
                        let temp_vol = tof64(&call_history[z+1]);
                        vol100+=temp_vol;
                        z+=4;
                        dem+=1;
                    }
                    hset(r,&pair.global,"high",&tostring(max_price100));
                    hset(r,&pair.global,"low",&tostring(min_price100));
                    hset(r,&pair.global,"vol",&tostring(vol100));
                    break 'price;
                    break 'phone;
                }
            }
        }
    }
}
fn event_cancel_order(r:&Connection,phone:&str,asset:&str,unit:&str,price:f64){
    let pair = Pair::new(asset,unit);
    let ph0ne = Phone::new(phone);
    let myorder = MyOrder::new(asset,unit,price);
    let myassets = MyAssets::new(asset);
    let myunits = MyAssets::new(unit);
    let side = &hget(r,&ph0ne.myorder,&myorder.side);
    let am = tof64(&hget(r,&ph0ne.myorder,&myorder.amount));
    let fi = tof64(&hget(r,&ph0ne.myorder,&myorder.fill));
    let du = am - fi;
    zincr(r,&myorder.price,&myorder.price,-du);
    if side == "BUY" {
        hincr(r,&ph0ne.myassets,&myunits.available,du*price);
        hincr(r,&pair.global,"amountbuy",-du);
    }
    if side == "SELL" {
        hincr(r,&ph0ne.myassets,&myassets.available,du);
        hincr(r,&pair.global,"amountsell",-du*price);
    }
    let timestamp = &tostrings(unix());
    let myhistory = MyOrder::newhistory(timestamp);
    add_history(r,&ph0ne,myhistory,&myorder);
    delete_order(r,&ph0ne,&myorder);
    let xx = zrange_withscores(r,&myorder.price,0,0)[0].1;
    if xx == 0.0 {
        del(r,&myorder.price);
    } else {
        zrem(r,&myorder.price,phone);
    }
}
fn claim(r:&Connection,phone:&str,asset:&str){
    let ph0ne = Phone::new(phone);
    let myassets = MyAssets::new(asset);
    hincr(r,&ph0ne.myassets,&myassets.available,1000.0);
    hincr(r,&ph0ne.myassets,&myassets.owned,1000.0);
}
//Main
fn main()-> redis::RedisResult<()>{
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    let r = &con;

    Ok(())
}

