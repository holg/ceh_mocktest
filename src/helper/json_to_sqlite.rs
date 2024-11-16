
#![cfg(any(feature = "use_sqlite", feature = "use_db"))]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use anyhow;
#[cfg(feature = "use_sqlite")]
use rusqlite;  // ::{Connection, Result, params};
use crate::questions::Question;

#[cfg(feature = "use_sqlite")]
pub fn json_to_sqlite() -> anyhow::Result<()> {
    // Read JSON file
    let file = File::open("questions.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let questions: Vec<Question> = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // Connect to SQLite database (or create if it doesn't exist)
    let mut conn = rusqlite::Connection::open("questions.db")?;

    // Enable JSON support
    conn.execute("PRAGMA journal_mode=WAL", [])?;

    // Create table with JSON column
    conn.execute(
        "CREATE TABLE IF NOT EXISTS questions (
            id INTEGER PRIMARY KEY,
            data JSON NOT NULL
        )",
        [],
    )?;

    // Insert questions
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare("INSERT INTO questions (data) VALUES (?)")?;
        for question in questions {
            let json = serde_json::to_string(&question)?;
            stmt.execute(rusqlite::params![json])?;
        }
    }
    tx.commit()?;

    println!("Questions successfully inserted into SQLite database using JSON format.");

    // Example query to demonstrate JSON functionality
    let mut stmt = conn.prepare("SELECT json_extract(data, '$.question') as question, json_extract(data, '$.answer') as answer FROM questions LIMIT 5")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    println!("\nExample of first 5 questions and answers:");
    for row in rows {
        let (question, answer) = row?;
        println!("Q: {}\nA: {}\n", question, answer);
    }

    Ok(())
}