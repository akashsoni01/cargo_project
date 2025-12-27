use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    id: Option<i64>,
    dect: Option<NaiveDateTime>,
    csrf: Vec<u8>,
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS test (
            id INTEGER PRIMARY KEY,
            dect TEXT NOT NULL,
            csrf BLOB NOT NULL
        )",
        (),
    )?;
    Ok(())
}

fn insert_test(conn: &Connection, test: &Test) -> Result<usize> {
    conn.execute(
        "INSERT INTO test (dect, csrf) VALUES (?1, ?2)",
        params![test.dect.as_ref().map(|d| d.to_string()), test.csrf],
    )
}

fn read_tests(conn: &Connection) -> Result<Vec<Test>> {
    let mut stmt = conn.prepare("SELECT id, dect, csrf FROM test")?;
    let test_iter = stmt.query_map((), |row| {
        Ok(Test {
            id: row.get(0)?,
            dect: row.get::<_, String>(1)?.parse::<NaiveDateTime>().ok(),
            csrf: row.get(2)?,
        })
    })?;

    let mut tests = Vec::new();
    for test in test_iter {
        tests.push(test?);
    }
    Ok(tests)
}

fn update_test(conn: &Connection, test: &Test) -> Result<usize> {
    conn.execute(
        "UPDATE test SET dect = ?1, csrf = ?2 WHERE id = ?3",
        params![
            test.dect.as_ref().map(|d| d.to_string()),
            test.csrf,
            test.id
        ],
    )
}

fn delete_test(conn: &Connection, id: i64) -> Result<usize> {
    conn.execute("DELETE FROM test WHERE id = ?1", params![id])
}

fn main() -> Result<()> {
    let conn = Connection::open("test.db")?;
    create_table(&conn)?;

    let new_test = Test {
        id: None,
        dect: Some("2023-06-17T00:00:00".parse().unwrap()),
        csrf: vec![1, 2, 3, 4],
    };

    insert_test(&conn, &new_test)?;

    let tests = read_tests(&conn)?;
    println!("Tests: {:?}", tests);

    let updated_test = Test {
        id: Some(1),
        dect: Some("2023-06-18T00:00:00".parse().unwrap()),
        csrf: vec![4, 3, 2, 1],
    };

    update_test(&conn, &updated_test)?;

    let tests = read_tests(&conn)?;
    println!("Updated Tests: {:?}", tests);

    delete_test(&conn, 1)?;

    let tests = read_tests(&conn)?;
    println!("Final Tests: {:?}", tests);

    Ok(())
}