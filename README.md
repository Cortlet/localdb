# 📦 localdb — Local SQL-Style JSON Database for Rust

`localdb` is a lightweight, file-based database engine built for Rust developers who want simple SQL-like commands without running a server.  
It stores all data locally in a JSON file, making it perfect for desktop apps, CLIs, tools, prototypes, and offline-first applications.

---

## 🚀 Features

- No server required — data stored in a single `.db` JSON file
- Supports:
    - `CREATE TABLE`
    - `INSERT INTO`
    - `SELECT * FROM table`
- Clean Rust API
- Safe error handling
- UUID, TEXT, and INT primitive types
- Fast + simple API
- Beginner-friendly

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
localdb = "0.1.0"
```

---

## 🛠️ Example Usage

```rust
use localdb::LocalDB;

fn main() {
    // Create DB
    let mut db = LocalDB::create("localdb.db").unwrap();

    // Add SQL lines
    let code = db.add_lines([
        "CREATE TABLE users (id UUID, name TEXT);",
        "INSERT INTO users VALUES ('11111111-1111-1111-1111-111111111111','kk');"
    ]);

    // Execute SQL
    db.exec(code).unwrap();

    // Query data
    let rows = db.query("SELECT * FROM users;").unwrap();

    println!("{:#?}", rows);
}
```

### Output:

```json
{
  "users": [
    {
      "id": { "UUID": "11111111-1111-1111-1111-111111111111" },
      "name": { "TEXT": "kk" }
    }
  ]
}
```

---

## 📚 Supported SQL Syntax

### ✔ CREATE TABLE
```
CREATE TABLE users (id UUID, name TEXT);
```

### ✔ INSERT INTO
```
INSERT INTO users VALUES ('uuid-here', 'name-here');
```

### ✔ SELECT
```
SELECT * FROM users;
```

---

## 🧩 File Format

`localdb` stores everything inside a JSON file like:

```json
{
  "users": [
    { "id": { "UUID": "..." }, "name": { "TEXT": "..." } }
  ]
}
```

---

## 🛡️ Error Handling

Every function returns a custom `LocalDBError`:

- I/O errors
- SQL syntax errors
- Table not found
- Invalid query format

---

## 📄 License

Apache-2.0  
Free for commercial + open source use.

---

## 👤 Author

Created by **Cortlet** — engineering the future of software.
