use docdb::{DocDb, DumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Serialize, Deserialize)]
struct Rectangle {
    width: i32,
    length: i32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle: length={}, width={}", self.length, self.width)
    }
}

fn main() {
    let db_name = "hello_world";

    let mut db = DocDb::new(db_name, DumpPolicy::AutoDump, SerializationMethod::Json);

    // set value
    db.set("num", &100).unwrap();
    db.set("float", &3.14).unwrap();
    db.set("str", &"string").unwrap();
    db.set("vec", &vec![1, 2, 3]).unwrap();

    db.set(
        "struct",
        &Rectangle {
            width: 2,
            length: 3,
        },
    )
    .unwrap();

    // print the vlaue of num
    println!("the value of num is {}", db.get::<i32>("num").unwrap());
    println!("the value of float is {}", db.get::<f64>("float").unwrap());
    println!("the value of str is {}", db.get::<String>("str").unwrap());
    println!(
        "the value of vec is {:?}",
        db.get::<Vec<i32>>("vec").unwrap()
    );
    println!(
        "the value of struct is {}",
        db.get::<Rectangle>("struct").unwrap()
    );

    // override num
    db.set("num", &"override").unwrap();
    println!(
        "get the vaule of num after overide: {}",
        db.get::<String>("num").unwrap()
    );

    // rem float
    db.rem("float").unwrap();
    println!(
        "float key is rem, if is this still eixst? {}",
        db.get::<f64>("float").is_some()
    );

    // load the exists db for file
    let db2 = DocDb::load(
        db_name,
        DumpPolicy::DumpRelyRequest,
        SerializationMethod::Json,
    )
    .unwrap();

    // verify the key of num
    println!(
        "verify the key of num is overide: {}",
        db2.get::<String>("num").unwrap()
    );

    // iterate over all keys
    for kv in db.iter() {
        match kv.get_key() {
            "num" => println!(
                "value of {} is: {}",
                kv.get_key(),
                kv.get_value::<String>().unwrap()
            ),

            "float" => println!(
                "value of {} is: {}",
                kv.get_key(),
                kv.get_value::<f64>().unwrap()
            ),

            "struct" => println!(
                "value of {} is: {}",
                kv.get_key(),
                kv.get_value::<Rectangle>().unwrap()
            ),
            _ => (),
        }
    }
}
