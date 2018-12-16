use std::io;
use std::collections::HashMap;

fn main() {

    let mut comp_dir = HashMap::new();

    loop {
        println!("Please enter employees name:");

        let mut employee = String::new();

        io::stdin().read_line(&mut employee)
            .expect("Failed to read line");

        let employee: String = employee.trim().to_string();

        println!("Enter their deparment:");

        let mut department = String::new();

        io::stdin().read_line(&mut department)
            .expect("Failed to read line");

        let department: String = department.trim().to_string();

        comp_dir.entry(department).or_insert(Vec::new()).push(employee);

        loop {
            println!("What department would you like to see the employees in?");

            let mut get_department = String::new();

            io::stdin().read_line(&mut get_department)
                .expect("Failed to read line");

            let get_department: String = get_department.trim().to_string();

            match comp_dir.get(&get_department) {
                Some(employees) => {
                    println!("That department has employees {:#?}", employees);
                    break;
                },
                None => {
                    println!("That department doesn't exist! Here is a list of departments to pick from: {:#?}", comp_dir.keys());
                },
            }
        }
    }
}

