use std::io;

// Employee struct
struct Employee {
    employee_name: String,
    employee_id: i32,
    email: String,
    age: i32,
    phone_number: String,
}

// Function to search for an employee by ID
fn find_employee_by_id(employees: &[Employee], target_id: i32) -> Option<&Employee> {
    employees.iter().find(|&employee| employee.employee_id == target_id)
}

// Function to get employees with the same age
fn find_employees_by_age(employees: &[Employee], target_age: i32) -> Vec<&Employee> {
    employees.iter().filter(|&employee| employee.age == target_age).collect()
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    println!("Enter the number of employees:");
    let mut num_employees = String::new();
    io::stdin().read_line(&mut num_employees).expect("Failed to read line");
    let num_employees: i32 = num_employees.trim().parse().expect("Please enter a valid number");

    for i in 0..num_employees {
        println!("\nEmployee {}:", i + 1);
        let mut employee_name = String::new();
        println!("Name:");
        io::stdin().read_line(&mut employee_name).expect("Failed to read line");
        let employee_name = employee_name.trim().to_string();

        let mut employee_id = String::new();
        println!("ID:");
        io::stdin().read_line(&mut employee_id).expect("Failed to read line");
        let employee_id: i32 = employee_id.trim().parse().expect("Please enter a valid number");

        let mut email = String::new();
        println!("Email:");
        io::stdin().read_line(&mut email).expect("Failed to read line");
        let email = email.trim().to_string();

        let mut age = String::new();
        println!("Age:");
        io::stdin().read_line(&mut age).expect("Failed to read line");
        let age: i32 = age.trim().parse().expect("Please enter a valid number");

        let mut phone_number = String::new();
        println!("Phone Number:");
        io::stdin().read_line(&mut phone_number).expect("Failed to read line");
        let phone_number = phone_number.trim().to_string();

        employees.push(Employee {
            employee_name,
            employee_id,
            email,
            age,
            phone_number,
        });
    }

    let mut search_id = String::new();
    println!("\nEnter the employee ID to search:");
    io::stdin().read_line(&mut search_id).expect("Failed to read line");
    let search_id: i32 = search_id.trim().parse().expect("Please enter a valid number");

    if let Some(found_employee) = find_employee_by_id(&employees, search_id) {
        println!("\nEmployee found:");
        println!("Name: {}", found_employee.employee_name);
        println!("ID: {}", found_employee.employee_id);
        println!("Email: {}", found_employee.email);
        println!("Age: {}", found_employee.age);
        println!("Phone Number: {}", found_employee.phone_number);
    } else {
        println!("\nEmployee not found.");
    }

    let mut search_age = String::new();
    println!("\nEnter the age to search employees:");
    io::stdin().read_line(&mut search_age).expect("Failed to read line");
    let search_age: i32 = search_age.trim().parse().expect("Please enter a valid number");

    let employees_with_same_age = find_employees_by_age(&employees, search_age);
    if !employees_with_same_age.is_empty() {
        println!("\nEmployees with age {}:", search_age);
        for employee in employees_with_same_age {
            println!("Name: {}", employee.employee_name);
            println!("ID: {}", employee.employee_id);
            println!("Email: {}", employee.email);
            println!("Age: {}", employee.age);
            println!("Phone Number: {}", employee.phone_number);
            println!();
        }
    } else {
        println!("\nNo employees with the given age.");
    }    
