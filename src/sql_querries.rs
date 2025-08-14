pub fn init_db() -> Result<()> {
    let conn = Connection::open("task_manager.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            due_date TEXT,
            status TEXT DEFAULT 'Pending'
        )",
        [],
    )?;
    Ok(())
}
