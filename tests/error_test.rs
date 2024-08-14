use fs2::FileExt;
use std::fs::File;

use docdb::error;
use docdb::{DocDb, DumpPolicy};
mod common;

#[test]
fn test_load_serialization_error() {
    set_test_src!("json_db.db");

    // create a new db with json serialization
    let mut db = DocDb::new_json("json_db.db", DumpPolicy::AutoDump);

    // set some values
    db.set("num_test", &10).unwrap();
    db.set("float_test", &3.14).unwrap();
    db.set("string_test", &"stringslice").unwrap();
    db.set("list", &vec![1, 2, 3]).unwrap();

    // try to load the same db with bincode serialization, should fail
    let load_as_bin = DocDb::load_bin("json_db.db", DumpPolicy::NeverDump);
    assert!(load_as_bin.is_err());

    let load_as_bin_err = load_as_bin.err().unwrap();
    // assert!(load_as_bin_err, error::DocError::IO());
    assert_eq!(
        load_as_bin_err.to_string(),
        "Deserialization err: cannot deserialize from db"
    );

    // try to load the smae db with YAML serialization, should not fail because YAML is a superset JSON object
    assert!(DocDb::load_yaml("json_db.db", DumpPolicy::NeverDump).is_ok());
}

#[test]
fn test_load() {
    let load_ret = DocDb::load_bin("not_exists.db", DumpPolicy::NeverDump);
    assert!(load_ret.is_err());

    let load_err_val = load_ret.err().unwrap();
    assert!(matches!(load_err_val.get_type(), error::ErrorType::IO))
}

#[test]
fn test_dump() {
    set_test_src!("dump_error_test.db");

    // only win os can lock file
    if cfg!(not(target_os = "windows")) {
        return;
    }

    // create a db with json
    let mut db = DocDb::new_json("dump_error_test.db", DumpPolicy::AutoDump);

    // set some values
    db.set("num_test", &10).unwrap();
    db.set("float_test", &3.14).unwrap();
    db.set("string_test", &"stringslice").unwrap();
    db.set("list", &vec![1, 2, 3]).unwrap();

    // lock db file
    let db_file = File::open("dump_error_test.db").unwrap();
    // fs2 function
    db_file.lock_exclusive().unwrap();

    // try set, confirm failure
    let try_set = db.set("num_test", &10);
    assert!(try_set.is_err());
    let try_set_err = try_set.err().unwrap();
    assert!(matches!(try_set_err.get_type(), error::ErrorType::IO));
    // verify old value still exist
    assert_eq!(db.get::<i32>("num_test").unwrap(), 10);

    // try dump, confirm failure
    let try_dump = db.dump();
    assert!(try_dump.is_err());
    let try_dump_err = try_dump.err().unwrap();
    assert!(matches!(try_dump_err.get_type(), error::ErrorType::IO));
    // verify old value still exist
    assert_eq!(db.get::<i32>("num_test").unwrap(), 10);

    // try rem, confirm failure
    let try_rem = db.rem("num_test");
    assert!(try_rem.is_err());
    let try_rem_err = try_rem.err().unwrap();
    assert!(matches!(try_rem_err.get_type(), error::ErrorType::IO));
    // verify old value still exist
    assert_eq!(db.get::<i32>("num_test").unwrap(), 10);

    // unlock the file
    db_file.unlock().unwrap();
}
