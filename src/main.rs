use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

// struct Alumno {
//     curp: String,
//     grupo: String,
//     nombre: String,
//     esp: i32,
//     ing: i32,
//     art: i32,
//     mat: i32,
//     bio: i32,
//     geo: i32,
//     his: i32,
//     fce: i32,
//     tec: i32,
//     efi: i32,
//     rap: i32,
//     pge: f32,
//     mom: i32,
// }


fn main() -> Result<()> {
    // let db: &str = "~/codelab/data/db/db_siie.db";
    // let conn = Connection::open(db);
    
    println!("Conectando con SQLite");

    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE person (
                id      INTEGER PRIMARY KEY,
                name    TEXT NOT NULL,
                data    BLOB
            )",
            (),
    )?;

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)", 
        (&me.name, &me.data)
    )?;

    let you = Person {
        id: 1,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)", 
        (&you.name, &you.data)
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person: {:?}", person.unwrap());
    }

    Ok(())
}
