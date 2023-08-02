#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let emp = Employee {
        name: String::from("John"),
        company: String::from("Google"),
        age: 35
    };

    println!("{:?}", emp);
    println!("{}", emp.name);
    println!("{}", emp.fn_details());
    println!("{}", Employee::static_fn_detail());
}

#[derive(Debug)]
struct Employee {
    name: String,
    company: String,
    age: u32
}

impl Employee {
    fn fn_details(&self) -> String {
        format!("name: {}, age: {}, company: {} ", &self.name, &self.age, &self.company)
    }

    fn static_fn_detail() -> String {
        String::from("Details of a person")
    }
}
