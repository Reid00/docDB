mod common;

use std::{thread, time::Duration};

use docdb::{DocDb, DumpPolicy, SerializationMethod};

#[test]
fn test_auto_dump() {
    let db_name = "auto_dump.db";

    // create a db with auto_dump == true
    let mut db = DocDb::new(&db_name, DumpPolicy::AutoDump, SerializationMethod::Yaml);

    assert!(db.set("num", &1).is_ok());

    // verify change in DB
    {
        let read_db = DocDb::load_read_only(db_name, SerializationMethod::Yaml).unwrap();
        assert_eq!(read_db.get::<i32>("num").unwrap(), 1);
    }

    // rem the key
    assert!(db.rem("num").unwrap_or(false));
    // verify change in DB
    {
        let read_db = DocDb::load_read_only(db_name, SerializationMethod::Yaml).unwrap();
        assert!(read_db.get::<i32>("num").is_none());
    }
}

#[test]
fn test_read_only_policy() {
    let db_name = "read_only.db";

    // create a db with read_only == true
    let mut db = DocDb::new(&db_name, DumpPolicy::AutoDump, SerializationMethod::Json);
    assert!(db.set("key", &String::from("this is key's val")).is_ok());

    let mut read_db = DocDb::load_read_only(db_name, SerializationMethod::Json).unwrap();

    assert!(read_db.set("key2", &"val2").is_ok());
    assert!(read_db.exist("key2"));

    // verify the set doesn't dump
    {
        let read_db2 = DocDb::load_read_only(db_name, SerializationMethod::Json).unwrap();
        assert!(read_db2.exist("key"));
        assert!(!read_db2.exist("key2"));
    }

    // try to dump db to the file, in fact just return ok()
    assert!(read_db.dump().is_ok());

    // verify the change doesn't dump
    {
        let read_db2 = DocDb::load_read_only(db_name, SerializationMethod::Json).unwrap();
        assert!(read_db2.exist("key"));
        assert!(!read_db2.exist("key2"));
    }

    drop(read_db);
    // verify the change doesn't dump
    {
        let read_db2 = DocDb::load_read_only(db_name, SerializationMethod::Json).unwrap();
        assert!(read_db2.exist("key"));
        assert!(!read_db2.exist("key2"));
    }
}

#[test]
fn test_rely_on_request_dump() {
    let db_name = "rely_on_request.db";

    // create a db with rely_on_request == true
    let mut db = DocDb::new(
        &db_name,
        DumpPolicy::DumpRelyRequest,
        SerializationMethod::Bin,
    );
    assert!(db.set("key", &String::from("this is key's val")).is_ok());

    // verify the db file not create yet
    assert!(DocDb::load_read_only(db_name, SerializationMethod::Bin).is_err());

    // dump file manully
    assert!(db.dump().is_ok());

    // verify the change is dumped to the file
    {
        let read_db = DocDb::load_read_only(db_name, SerializationMethod::Bin).unwrap();
        assert!(read_db.exist("key"));
    }

    // add more key
    db.set("key2", &1).unwrap();

    // drop db
    drop(db);

    // verify the key2 doesn't dumped
    {
        let read_db = DocDb::load_read_only(db_name, SerializationMethod::Bin).unwrap();
        assert!(read_db.exist("key"));
        assert!(!read_db.exist("key2"));
    }
}

#[test]
fn test_periodic_dump_policy() {
    let db_name = "periodid_db.db";

    // create a db with periodid_db == true
    let mut db = DocDb::new(
        &db_name,
        DumpPolicy::PeriodicDump(Duration::new(1, 0)),
        SerializationMethod::Bin,
    );
    assert!(db.set("key", &String::from("this is key's val")).is_ok());

    // verify the dump file not create yet
    assert!(DocDb::load_read_only(db_name, SerializationMethod::Bin).is_err());

    // sleep 0.5s
    thread::sleep(Duration::from_millis(500));

    // verify the dump file still not create yet
    assert!(DocDb::load_read_only(db_name, SerializationMethod::Bin).is_err());

    // sleep 0.55s again
    thread::sleep(Duration::from_millis(550));

    db.set("key2", &1).unwrap();
    // verify the file is created
    {
        let read_db = DocDb::load_read_only(db_name, SerializationMethod::Bin).unwrap();
        assert!(read_db.exist("key"));
        assert!(read_db.exist("key2"));
    }

    // add another key
    db.set("key3", &"val3").unwrap();
    // verify the key doesn't dump
    {
        let read_db = DocDb::load_read_only(db_name, SerializationMethod::Bin).unwrap();
        assert!(read_db.exist("key"));
        assert!(read_db.exist("key2"));
        assert!(!read_db.exist("key3"));
    }

    // add new key
    db.set("key4", &"val4").unwrap();

    // drop db
    drop(db);

    // verify key4 is dumped
    {
        let read_db = DocDb::load_read_only(db_name, SerializationMethod::Bin).unwrap();
        assert!(read_db.exist("key"));
        assert!(read_db.exist("key4"));
    }
}
