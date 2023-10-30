# Rust SQLite CLI 
In these project we implement a Result CLI tool that interacts with an SQLite database

### Main Logic of the CLI
```rust

pub fn start() {
    let conn = connect_db("students.db").unwrap();

    loop {
        print_menu();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u8>().unwrap() {
            1 => create_student_cli(&conn),
            2 => get_students_cli(&conn),
            // 3 => update_student_cli(&conn),
            4 => delete_student_cli(&conn),
            5 => {
                println!("Bye!");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}

fn print_menu() {
    println!();
    println!("1. Create student");
    println!("2. Get students");
    println!("3. Update student");
    println!("4. Delete student");
    println!("5. Quit");
    println!();
}

fn create_student_cli(conn: &rusqlite::Connection) {
    let name = prompt_string("Enter name: ");
    let major = prompt_string("Enter major: ");
    let grad_year = &prompt_string("Enter graduation year: ");
    
    match create_student(conn, &name, &major, grad_year) {
        Ok(_) => println!("Inserted student successfully"),
        Err(e) => println!("Error inserting student: {}", e),
    }
}

fn get_students_cli(conn: &rusqlite::Connection) {
   match get_students(conn) {
       Ok(students) => {
           println!("{:-^20}", "STUDENTS");
           for student in students {
               println!("{} (major: {}, grad year: {})", 
                        student.name, student.major, student.grad_year);
           }
           println!("{:-^20}", "");
       },
       Err(e) => println!("Error getting students: {}", e),
   }
}

// fn update_student_cli(conn: &rusqlite::Connection) {
//     let id = prompt_int("Enter student ID to update: ");
//     let name = prompt_string("Enter new name: ");
//     let major = prompt_string("Enter new major: ");
//     let grad_year = prompt_int("Enter new graduation year: ");

//     match update_student_cli(conn, id, &name, &major, grad_year) {
//         Ok(_) => println!("Updated student successfully"),
//         Err(e) => println!("Error updating student: {}", e),
//     }
// }

fn delete_student_cli(conn: &rusqlite::Connection) {
    let id = prompt_int("Enter student ID to delete: ");
    
    match delete_student(conn, id) {
        Ok(_) => println!("Deleted student successfully"),
        Err(e) => println!("Error deleting student: {}", e),
    }
}

fn prompt_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_int(prompt: &str) -> i32 {
    let input = prompt_string(prompt);
    input.parse().unwrap()
}
```

### Example Usage
![Example](/images/cli.png?raw=true)

## Disclosure
Some of these code was generated using AI tools but I fully understand what is going on . 

### Additional 
We also created a postgres azure cloud database and provided the python connection and finalising the Rust CLI interaction with the cloud database
