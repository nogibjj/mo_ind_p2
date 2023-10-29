use rusqlite::{Connection, Result,params};

pub struct Student {
    pub id: i32,
    pub name: String,
    pub major: String,
    pub grad_year: i32,
}

pub fn connect_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS students (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             major TEXT NOT NULL,
             grad_year INTEGER NOT NULL
         )",
        [],
    )?;

    Ok(conn)
}

pub fn create_student(conn: &Connection, name: &str, major: &str, grad_year: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO students (name, major, gradyear) VALUES (?1, ?2, ?3)",
        [name, major, &grad_year],
    )?;
    
    Ok(())
}

pub fn get_students(conn: &Connection) -> Result<Vec<Student>> {
    let mut stmt = conn.prepare("SELECT id, name, major, grad_year FROM students")?;
    let students = stmt.query_map(params![], |row| {
        Ok(Student {
            id: row.get(0)?,
            name: row.get(1)?,
            major: row.get(2)?, 
            grad_year: row.get(3)?
        })
    })?;

    let mut students = students.map(|stu| stu.unwrap()).collect();
    Ok(students)
}

pub fn update_student(conn: &Connection, id: &str, name: &str, major: &str, grad_year: &str) -> Result<()> {

    conn.execute(
      "UPDATE students SET name = ?1, major = ?2, grad_year = ?3 WHERE id = ?4",
      [name, major, &grad_year, &id], // grad_year is now i32
    )?;
  
    Ok(()) 
  }

pub fn delete_student(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM students WHERE id = ?1", [&id])?;
    Ok(())
}