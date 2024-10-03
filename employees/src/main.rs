use std::collections::HashMap;

struct Department {
    dep: String,
    emp: Vec<String>
}

struct Company {
    deps: Vec<Department>
}

fn main() {
    // Hashmap to hold all employees with their attached department
    let mut employees = HashMap::new();
    employees.insert("Max", "sales");
    employees.insert("Sally", "sales");
    employees.insert("Felix", "enginnering");
    employees.insert("Hugo", "enginnering");

    // show_employees_for_department(&employees, "sales");
    show_employees_by_department_sorted(&employees);

}

fn _show_employees_for_department(employees: &HashMap<&str, &str>, department: &str) {
    println!{"These are the employees working in the department <{department}>"};
    for (emp, dep) in employees {
        if department == *dep {
            println!("{emp}");
        }
    }
}

fn show_employees_by_department_sorted(employees: &HashMap<&str, &str>) {
    println!{"\nThese are the employees per department, sorted alphabetically"};
    let mut deps: Vec<String> = Vec::new();
    let mut company = Company {
        deps : Vec::new()
    };

    let mut _emp: Vec<String> = Vec::new();
    for (emp, dep) in employees {
        // see here: https://users.rust-lang.org/t/how-to-check-if-string-exists-in-a-vec/63180/2
        if !deps.iter().any(|e|dep.contains(e)) {
            deps.push(dep.to_string());
            println!("Erzeuge neues Department {dep}");
            let mut department = Department {
                dep : dep.to_string(),
                emp : Vec::new()
            };
            department.emp.push(emp.to_string());
            &company.deps.push(department);
        } else {
            for d in company.deps {
                if (&d.dep == dep) {
                    
                }
            }
        }
    }
}
