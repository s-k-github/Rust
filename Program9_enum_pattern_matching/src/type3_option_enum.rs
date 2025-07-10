enum Employee {
    EmployeeID(i32),
    EmployeeFistLastName { first: String, last: String },
    EmployeeEmail(String),
    EmployeeJoiningDate(Option<String>),
}
impl Employee {
    fn option_type(&self) {
        match self {
            Employee::EmployeeID(x) => {
                println!("{}", x)
            }
            Employee::EmployeeFistLastName { first, last } => {
                println!("Hello, {} {}", first, last)
            }
            Employee::EmployeeEmail(s) => {
                let a = s;
                println!("{}", a)
            }
            Employee::EmployeeJoiningDate(Some(x)) => {
                println!("Joining Date : {}", x)
            }
            Employee::EmployeeJoiningDate(None) => {
                println!("Joining Date Not Provided")
            }
            _ => (),
        }
    }
}
pub fn option_enum() {
    println!("--------------------------------------------------------------------------------");
    let a = Employee::EmployeeFistLastName {
        first: String::from("Supriya"),
        last: String::from("Jadhav"),
    };
    a.option_type();
    let b = Employee::EmployeeJoiningDate(Some(String::from("10-Jul-2025")));
    b.option_type();
}
