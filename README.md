# docDB
docDB is a lightweight and simple key-value store. It is inspired Python's PickleDB

# How to Use
```rust
use  docdb::{DocDb, DumpPolicy, SerializationMethod};

fn main() {

    // create a new DB with AutoDump (meaning every change is written to the file)
    // and with Json serialization (meaning DB will be dumped to file as a Json object)
    let mut db = docdb::new("example.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);

    // set the value 100 to the key 'key1'
    db.set("key1", &100).unwrap();

    // print the value of key1
    println!("The value of key1 is: {}", db.get::<i32>("key1").unwrap());

    // load the DB from the same file
    let db2 = docdb::load("example.db", PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json).unwrap();

    // print the value of key1
    println!("The value of key1 as loaded from file is: {}", db2.get::<i32>("key1").unwrap());
}

```

# Installation
Add dependencies on cargo.toml
```rust
[dependencies]
docdb = "0.1.0"
```

