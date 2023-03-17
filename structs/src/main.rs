use uuid::Uuid;

#[derive(Debug)]
struct Customer {
    id: Uuid,
}

impl Customer {
    fn clear(self) {}
}

fn main() {
    let customer = Customer { id: Uuid::new_v4() };
    println!("{:?}", customer);
    println!("{}", customer.id);
    customer.clear();
}
