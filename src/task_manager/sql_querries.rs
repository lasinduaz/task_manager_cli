use rusqlite::{Connection, Result, params};

pub fn connecttion_() -> Result<Connection> {
    Connection::open("task_manager.db")
}

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

pub fn insert_task(conn: &Connection, title: &str, description: &str, due_date: &str, status: &str) -> Result<usize> {
    conn.execute(
        "INSERT INTO tasks (title, description, due_date, status) VALUES (?1, ?2, ?3, ?4)",
        params![title, description, due_date, status],
    )
}

pub fn view_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, title, description, due_date, status FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
        ))
    })?;
    for task in task_iter {
        let (id, title, description, due_date, status) = task?;
        println!("ID: {}, Title: {}, Description: {}, Due Date: {}, Status: {}", id, title, description, due_date, status);
    }
    Ok(())
}