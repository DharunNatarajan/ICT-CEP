use std::error::Error;

#[derive(Debug)]
struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut students: Vec<Student> = Vec::new();

    students.push(Student {
        name: String::from("Shaam"),
        email: String::from("shaam245@gmail.com"),
        phone: String::from("9850632108"),
        id: 1,
    });

    students.push(Student {
        name: String::from("Xavier"),
        email: String::from("xavierbritto96@gmail.com"),
        phone: String::from("9632580741"),
        id: 2,
    });

    // Add more students...

    // Accessing student by index
    let student_index = 1;
    if let Some(student) = students.get(student_index) {
        println!("Student {} details:", student_index);
        println!("Name: {}", student.name);
        println!("Email: {}", student.email);
        println!("Phone: {}", student.phone);
        println!("ID: {}", student.id);
    } else {
        eprintln!("Student not found for index {}", student_index);
    }

    // Attempting out of bounds access
    let invalid_index = 10;
    if let Some(student) = students.get(invalid_index) {
        println!("Student {} details:", invalid_index);
        println!("Name: {}", student.name);
        println!("Email: {}", student.email);
        println!("Phone: {}", student.phone);
        println!("ID: {}", student.id);
    } else {
        eprintln!("Student not found for index {}", invalid_index);
    }

    Ok(())
}
