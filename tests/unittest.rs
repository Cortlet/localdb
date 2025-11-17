use localdb::{LocalDB, LocalDBValue};
use std::fs;

#[test]
fn test_create_insert_select() {
    // create temporary DB
    let path = "test_localdb.db";

    // remove old file if exists
    let _ = fs::remove_file(path);

    let mut db = LocalDB::create(path).expect("failed to create DB");

    let sql = db.add_lines([
        "CREATE TABLE users (id UUID, name TEXT);",
        "INSERT INTO users VALUES ('11111111-1111-1111-1111-111111111111', 'kk');"
    ]);

    db.exec(sql).expect("SQL exec failed");

    let rows = db.query("SELECT * FROM users;").expect("query failed");

    // We expect exactly 1 row
    assert_eq!(rows.len(), 1);

    let row = &rows[0];

    // Check name
    match row.get("name").unwrap() {
        LocalDBValue::TEXT(name) => assert_eq!(name, "kk"),
        _ => panic!("name should be TEXT"),
    }

    // Check UUID
    match row.get("id").unwrap() {
        LocalDBValue::UUID(id) => assert_eq!(id, "11111111-1111-1111-1111-111111111111"),
        _ => panic!("id should be UUID"),
    }

    // cleanup
    let _ = fs::remove_file(path);
}
