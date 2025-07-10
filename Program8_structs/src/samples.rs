#[derive(Debug)]
pub struct CustomerMaster {
    cust_id: i32,
    cust_name: String,
    isActive: bool,
}
impl CustomerMaster {
    fn get_custid(&self) -> () {
        //() is optional. it means it returns unittype
        println!("{}, {}", self.cust_id, self.cust_name);
    }
    fn is_oldest_customer(&self, other: &CustomerMaster) -> bool {
        self.cust_id < other.cust_id
    }
}
pub fn immutable_struct() {
    let customer1 = CustomerMaster {
        cust_id: 12345,
        cust_name: String::from("World Fuel"),
        isActive: true,
    };
    println!("{}", customer1.cust_name);
}
pub fn mutable_struct() {
    let mut customer2 = CustomerMaster {
        cust_id: 12345,
        cust_name: String::from("World Fuel"),
        isActive: true,
    };

    customer2.cust_name = String::from("Act32");
    println!("{}", customer2.cust_name);
}
pub fn create_new_customer(cust_id: i32, cust_name: String, is_active: bool) -> CustomerMaster {
    CustomerMaster {
        cust_id,
        cust_name,
        isActive: is_active,
    }
}
pub fn use_struct_as_param() {
    let customer1 = CustomerMaster {
        cust_id: 567,
        cust_name: String::from("StructWithParam"),
        isActive: true,
    };
    struct_as_param(&customer1)
}
fn struct_as_param(customer1: &CustomerMaster) {
    println!("{:?}", customer1);
    println!("{:#?}", customer1);
    println!("{:#?}", customer1.get_custid());
    let customer2 = CustomerMaster {
        cust_id: 568,
        cust_name: String::from("StructWithParam"),
        isActive: true,
    };
    let customer3 = CustomerMaster {
        cust_id: 100,
        cust_name: String::from("StructWithParam"),
        isActive: true,
    };
    println!(
        "{:#?} is oldest than {:#?} ? {:#?} ",
        customer1.cust_id,
        customer2.cust_id,
        customer1.is_oldest_customer(&customer2)
    );
    println!(
        "{:#?} is oldest than {:#?} ? {:#?} ",
        customer1.cust_id,
        customer3.cust_id,
        customer1.is_oldest_customer(&customer3)
    );
}
