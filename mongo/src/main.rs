extern crate mongodb;
use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
macro_rules! insert_one {
    ($coll:path, $doc:path) => {
        $coll.insert_one($doc.clone(), None).ok().expect("Failed to insert document.");
    };
}
macro_rules! update_one {
    ($coll:path, $filter:path, $update:path) => {
        $coll.update_one($filter.clone(), $update.clone(), None).ok().expect("Failed to execute update one.");
    };
}
macro_rules! replace_one {
    ($coll:path, $filter:path, $replace:path) => {
        $coll.replace_one($filter.clone(), $replace.clone(), None).ok().expect("Failed to execute update one.");
    };
}
macro_rules! delete_one {
    ($coll:path, $filter:path) => {
        $coll.delete_one($filter.clone(), None).ok().expect("Failed to execute find one.");
    };
}
macro_rules! find_one {
    ($coll:path, $filter:path) => {
        $coll.find_one(Some($filter.clone()), None).ok().expect("Failed to execute find one.").unwrap()
    };
}
macro_rules! user {
    ($email:expr,) => {
        
    };
}
fn main() {
    let client = Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");
    let coll = client.db("digigo").collection("users");
}
