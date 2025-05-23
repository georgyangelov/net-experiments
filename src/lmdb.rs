use std::fs;
use heed::{Database, EnvOpenOptions};
use heed::types::{Str};

pub fn run() {
    let db_dir = "lmdb_db";

    if !fs::exists(db_dir).unwrap() {
        fs::create_dir(db_dir).expect("could not create DB dir");
    }

    let env = unsafe {
        EnvOpenOptions::new()
            .max_dbs(10)
            .open(db_dir)
            .expect("could not open DB")
    };

    let mut tx = env.write_txn().expect("could not open write transaction");

    let db: Database<Str, Str> = env.create_database(&mut tx, Some("testdb"))
        .expect("could not create db");

    db.put(&mut tx, "test", "42").unwrap();
    tx.commit().unwrap();

    let tx = env.read_txn().expect("could not open read tx");

    let result = db.get(&tx, "test").expect("could not get value")
        .expect("missing value")
        .to_string();
    tx.commit().unwrap();

    println!("Result: {result:?}");
}
