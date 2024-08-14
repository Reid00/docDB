use docdb::{DocDb, DumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};

mod common;

#[test]
fn test_basic_set() {
    let db_name = "test_basic.db";

    let mut db = DocDb::new(db_name, DumpPolicy::AutoDump, SerializationMethod::Json);

    // set a number
    let num = 10;
    let _ = db.set("num_test", &num);

    // set a float
    let _ = db.set("float_test", &3.14);

    // set a string
    let _ = db.set("string_test", &"my string");

    // set a Vec
    let myvec = vec![1, 2, 3];
    let _ = db.set("vec_test", &myvec);

    // set a struct
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y: 2 };
    let _ = db.set("struct", &mycoor);

    // read a num
    assert_eq!(db.get::<i32>("num_test").unwrap(), num);
    // read a floating point number
    assert_eq!(db.get::<f32>("float_test").unwrap(), 3.14);
    // read a String
    assert_eq!(
        db.get::<String>("string_test").unwrap(),
        "my string".to_string()
    );
    // read a Vec
    assert_eq!(db.get::<Vec<i32>>("vec_test").unwrap(), myvec);
    // read a struct
    assert_eq!(db.get::<Coor>("struct").unwrap().x, mycoor.x);
    assert_eq!(db.get::<Coor>("struct").unwrap().y, mycoor.y);
}

#[test]
fn test_laod_get() {
    let db_name = "test_load.db";

    let mut db = DocDb::new(
        db_name,
        DumpPolicy::DumpRelyRequest,
        SerializationMethod::Bin,
    );

    // set a number
    let num = 10;
    let _ = db.set("num_test", &num);

    // set a float
    let val = 3.14;
    let _ = db.set("float_test", &val);

    // set a string
    let _ = db.set("string_test", &"my string");

    // set a Vec
    let myvec = vec![1, 2, 3];
    let _ = db.set("vec_test", &myvec);

    // set a struct
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y: 2 };
    let _ = db.set("struct", &mycoor);

    // -------------------------------------
    // dump db to file
    assert!(db.dump().is_ok());

    // read db from file
    let read_db = DocDb::load_read_only(&db_name, SerializationMethod::Bin).unwrap();

    // read a num
    assert_eq!(read_db.get::<i32>("num_test").unwrap(), num);
    // read a floating point number
    assert_eq!(read_db.get::<f32>("float_test").unwrap(), val);
    // read a String
    assert_eq!(
        read_db.get::<String>("string_test").unwrap(),
        "my string".to_string()
    );
    // read a Vec
    assert_eq!(read_db.get::<Vec<i32>>("vec_test").unwrap(), myvec);
    // read a struct
    assert_eq!(read_db.get::<Coor>("struct").unwrap().x, mycoor.x);
    assert_eq!(read_db.get::<Coor>("struct").unwrap().y, mycoor.y);
}

#[test]
fn test_laod_get_autodump() {
    let db_name = "test_load_auto.db";

    let mut db = DocDb::new(db_name, DumpPolicy::AutoDump, SerializationMethod::Bin);

    // set a number
    let num = 10;
    let _ = db.set("num_test", &num);

    // set a float
    let val = 3.14;
    let _ = db.set("float_test", &val);

    // set a string
    let _ = db.set("string_test", &"my string");

    // set a Vec
    let myvec = vec![1, 2, 3];
    let _ = db.set("vec_test", &myvec);

    // set a struct
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y: 2 };
    let _ = db.set("struct", &mycoor);

    // -------------------------------------
    // dump db to file
    assert!(db.dump().is_ok());

    // read db from file
    let read_db = DocDb::load_read_only(&db_name, SerializationMethod::Bin).unwrap();

    // read a num
    assert_eq!(read_db.get::<i32>("num_test").unwrap(), num);
    // read a floating point number
    assert_eq!(read_db.get::<f32>("float_test").unwrap(), val);
    // read a String
    assert_eq!(
        read_db.get::<String>("string_test").unwrap(),
        "my string".to_string()
    );
    // read a Vec
    assert_eq!(read_db.get::<Vec<i32>>("vec_test").unwrap(), myvec);
    // read a struct
    assert_eq!(read_db.get::<Coor>("struct").unwrap().x, mycoor.x);
    assert_eq!(read_db.get::<Coor>("struct").unwrap().y, mycoor.y);
}

#[test]
fn test_special_string() {
    let db_name = "test_special_string.db";

    // create a db with auto_dump == true
    let mut db = DocDb::new(&db_name, DumpPolicy::AutoDump, SerializationMethod::Bin);

    db.set("string1", &String::from("\"double_quotes\""))
        .unwrap();
    db.set("string2", &String::from("\'single_quotes\'"))
        .unwrap();
    db.set("string3", &String::from("שָׁלוֹם")).unwrap();
    db.set("string4", &String::from("😻")).unwrap();
    db.set("string5", &String::from("\nescapes\t\r")).unwrap();
    db.set("string6", &String::from("my\\folder")).unwrap();

    let read_db = DocDb::load_read_only(&db_name, SerializationMethod::Bin).unwrap();
    assert_eq!(
        read_db.get::<String>("string1").unwrap(),
        String::from("\"double_quotes\"")
    );
    assert_eq!(
        read_db.get::<String>("string2").unwrap(),
        String::from("\'single_quotes\'")
    );
    assert_eq!(
        read_db.get::<String>("string3").unwrap(),
        String::from("שָׁלוֹם")
    );
    assert_eq!(
        read_db.get::<String>("string4").unwrap(),
        String::from("😻")
    );
    assert_eq!(
        read_db.get::<String>("string5").unwrap(),
        String::from("\nescapes\t\r")
    );
    assert_eq!(
        read_db.get::<String>("string6").unwrap(),
        String::from("my\\folder")
    );
}

#[test]
fn test_edge_cases() {
    let db_name = "test_edge_cases.db";

    // create a db with auto_dump == true
    let mut db = DocDb::new(&db_name, DumpPolicy::AutoDump, SerializationMethod::Bin);

    let x = 123;
    db.set("num", &x).unwrap();

    // load a read only version of the db from file
    let read_db = DocDb::load_read_only(&db_name, SerializationMethod::Bin).unwrap();

    assert_eq!(db.get::<i32>("num"), Some(x));
    assert_eq!(read_db.get::<i32>("num"), Some(x));

    assert_eq!(db.get::<String>("num"), None);
    assert_eq!(read_db.get::<String>("num"), None);
}

#[test]
fn test_get_all_keys() {
    let db_name = "test_get_all_keys.db";

    // create a db with auto_dump == true
    let mut db = DocDb::new(&db_name, DumpPolicy::AutoDump, SerializationMethod::Bin);

    // insert 10 keys: key0..key9
    let num = 100;
    for i in 0..10 {
        db.set(&format!("{}{}", "key", i), &num).unwrap();
    }

    // verify we have 10 keys
    assert_eq!(db.total_nums(), 10);

    // get all keys
    let keys = db.get_all_keys();

    // verify we got 10 keys
    assert_eq!(keys.len(), 10);

    // verify all key names are there
    for i in 0..9 {
        assert!(keys.iter().any(|key| key == &format!("{}{}", "key", i)));
    }
}

#[test]
fn test_rem_keys() {
    let db_name = "test_rem.db";

    // create a db with auto_dump == true
    let mut db = DocDb::new(&db_name, DumpPolicy::AutoDump, SerializationMethod::Bin);

    let num = 10;
    for i in 0..10 {
        db.set(&format!("{}{}", "key", i), &num).unwrap();
    }

    // remove 2 keys
    assert!(db.rem("key3").unwrap_or(false));
    assert!(db.rem("key9").unwrap_or(false));

    // verify only 8 keys
    assert_eq!(db.total_nums(), 8);

    // verify both keys were removed
    for i in vec![3, 9] {
        assert!(!db.exist(&format!("{}{}", "key", i)))
    }

    // verify other key still exist
    for i in vec![0, 1, 2, 4, 5, 6, 7, 8] {
        assert!(db.exist(&format!("{}{}", "key", i)))
    }

    let read_db = DocDb::load_read_only(&db_name, SerializationMethod::Bin).unwrap();
    assert_eq!(read_db.total_nums(), 8);
}

#[test]
fn test_iter() {
    let db_name = "test_iter.db";

    // create a db with auto_dump == true
    let mut db = DocDb::new(&db_name, DumpPolicy::AutoDump, SerializationMethod::Bin);

    let keys = vec!["1", "2", "3", "4", "5"];
    // add key and value
    db.set(keys[0], &0).unwrap();
    db.set(keys[1], &1.1).unwrap();
    db.set(keys[2], &"val".to_string()).unwrap();
    db.set(keys[3], &vec![1, 2, 3]).unwrap();
    db.set(keys[4], &('a', 'b', 'c')).unwrap();

    let mut key_seen = vec![false, false, false, false, false];
    for key_val in db.iter() {
        let idx = keys.iter().position(|&k| k == key_val.get_key()).unwrap();

        key_seen[idx] = true;

        match key_val.get_key() {
            "1" => assert_eq!(key_val.get_value::<i32>().unwrap(), 0),
            "2" => assert_eq!(key_val.get_value::<f64>().unwrap(), 1.1),
            "3" => assert_eq!(key_val.get_value::<String>().unwrap(), "val".to_string()),
            "4" => assert_eq!(key_val.get_value::<Vec<i32>>().unwrap(), vec![1, 2, 3]),
            "5" => assert_eq!(
                key_val.get_value::<(char, char, char)>().unwrap(),
                ('a', 'b', 'c')
            ),
            _ => panic!(),
        }
    }

    // verify all 5 keys were seen
    assert_eq!(key_seen.iter().filter(|&t| *t).count(), 5);
}
