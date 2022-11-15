struct Employees {
    employees: Vec<Employee>,
}

struct Employee {
    name: String,
    id: u16,
    salary: f32,
}


impl Employees {
    fn new_employees() -> Self {
        return Employees {
            employees: Vec::new()
        };
    }

    fn find_by(&self, id: u16) -> Option<&Employee> {
        for e in &self.employees {
            if e.id == id {
                return Some(e);
            }
        }
        return None;
    }

    fn find_by_predicate<F>(&self, predicate: F) -> Option<&Employee>
        where F: Fn(&Employee) -> bool
    {
        for e in &self.employees {
            if predicate(e) {
                return Some(e);
            }
        }
        return None;
    }

    fn add(&mut self, employee: Employee) {
        self.employees.push(employee);
    }
}

#[test]
fn test_find_employees_by_existing_id_1() {
    let mut employees = Employees::new_employees();
    employees.add(Employee { name: "A".to_string(), id: 1, salary: 100.34 });
    employees.add(Employee { name: "B".to_string(), id: 2, salary: 100.34 });

    let found = employees.find_by(1);
    assert_eq!(1, found.unwrap().id);
    assert_eq!("A", found.unwrap().name);
}

#[test]
fn test_find_employees_by_existing_id_2() {
    let mut employees = Employees::new_employees();
    employees.add(Employee { name: "A".to_string(), id: 1, salary: 100.34 });
    employees.add(Employee { name: "B".to_string(), id: 2, salary: 100.34 });

    let found = employees.find_by(2);
    assert_eq!(2, found.unwrap().id);
    assert_eq!("B", found.unwrap().name);
}

#[test]
fn test_find_employees_by_a_non_existing_id() {
    let mut employees = Employees::new_employees();
    employees.add(Employee { name: "A".to_string(), id: 1, salary: 100.34 });

    let found = employees.find_by(20);
    assert_eq!(true, found.is_none())
}

#[test]
fn test_find_employees_by_salary_gt() {
    let mut employees = Employees::new_employees();
    employees.add(Employee { name: "A".to_string(), id: 1, salary: 200.34 });
    employees.add(Employee { name: "B".to_string(), id: 2, salary: 100.34 });

    let found = employees.find_by_closure(|e: &Employee| -> bool {
        return e.salary > 100.34;
    });
    assert_eq!(1, found.unwrap().id);
    assert_eq!("A", found.unwrap().name);
    assert_eq!(200.34, found.unwrap().salary);
}

