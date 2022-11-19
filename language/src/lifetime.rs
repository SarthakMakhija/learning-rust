struct Employees<'b> {
    employees: Vec<Employee<'b>>,
}

struct Employee<'a> {
    name: &'a str,
    id: u16,
}


impl<'a> Employees<'a> {
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

    fn add(&mut self, employee: Employee<'a>) {
        self.employees.push(employee);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_employees_by_existing_id_1() {
        let mut employees = Employees::new_employees();
        employees.add(Employee { name: "A", id: 1 });

        let found = employees.find_by(1);
        assert_eq!(1, found.unwrap().id);
        assert_eq!("A", found.unwrap().name);
    }

    #[test]
    fn test_find_employees_by_existing_id_2() {
        let mut employees = Employees::new_employees();
        employees.add(Employee { name: "A", id: 1 });
        employees.add(Employee { name: "B", id: 2 });

        let found = employees.find_by(2);
        assert_eq!(2, found.unwrap().id);
        assert_eq!("B", found.unwrap().name);
    }

    #[test]
    fn test_find_employees_by_a_non_existing_id() {
        let mut employees = Employees::new_employees();
        employees.add(Employee { name: "A", id: 1 });

        let found = employees.find_by(20);
        assert_eq!(true, found.is_none())
    }
}