// create an array of struct = employee name, age, salary
// push employee data
// query name return your struct


use std::num::IntErrorKind::Empty;

// #[derive(Debug, Copy)]
struct Employe {
    name: String,
    age: i32,
    salary: f64
}

struct  Employes {
    employes: Vec<Employe>
}
impl  Employes {
    fn new(employes: Vec<Employe>) -> Self {
        Self { employes }
    }


    fn query(&self, search: String) -> Option<Employe> {
        for employe in self.employes {
            if employe.name == search {
                return Some(employe);
            }
        }
        None
    }

}
fn main() {
    let mut array_of_employee = Employes::new(vec![
        Employe{
            name: "emp1".to_string(),
            age: 12,
            salary: 12.2
        }
    ]);

    array_of_employee.employes.push(Employe{
        name: "emp2".to_string(),
        age: 12,
        salary: 12.2
    });


    array_of_employee.employes.push(Employe{
        name: "emp3".to_string(),
        age: 12,
        salary: 12.2
    });


    array_of_employee.employes.push(Employe{
        name: "emp4".to_string(),
        age: 12,
        salary: 12.2
    });


    if let Some(query_emp) = array_of_employee.query("emp4".to_string()) {
        println!("query employe = {}", query_emp.name);
    }
}

/**/

enum MyOption<T>{
    Some(T),
    None
}